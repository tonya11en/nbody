use std::error::Error;
use std::thread;

use log::{debug, info, trace, warn};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Point, Vec3d};

#[derive(Serialize, Deserialize, Debug)]
pub struct BHTree {
    root: BHNode,
    theta: f64,
    graph_size: f64,
}

impl BHTree {
    pub fn new(theta: f64, graph_size: f64, x: f64, y: f64, z: f64) -> BHTree {
        info!(theta = theta, graph_size = graph_size; "creating barnes-hut tree");
        return BHTree {
            root: BHNode::new(theta, graph_size, x, y, z),
            theta: theta,
            graph_size: graph_size,
        };
    }

    pub fn add_point(&mut self, p: Point) {
        trace!("adding point {}", p);
        self.root.add_point(p);
    }

    pub fn next(&self, dt: f64) -> BHTree {
        debug!("creating next bht...");

        debug!("creating new point set");
        let new_points_iter: Vec<_> = self
            .root
            .get_points()
            .par_iter()
            .map(|p| {
                let force = self.root.calculate_force(dt, *p);
                return p.apply_force(dt, force);
            })
            .collect();

        let mut min_dim = f64::MAX;
        let mut max_dim = f64::MIN;
        for p in &new_points_iter {
            let (x, y, z) = p.position();
            min_dim = x.min(min_dim);
            max_dim = x.max(max_dim);
            min_dim = y.min(min_dim);
            max_dim = y.max(max_dim);
            min_dim = z.min(min_dim);
            max_dim = z.max(max_dim);
        }
        max_dim += 1.;
        min_dim -= 1.;

        let graph_size = max_dim - min_dim;
        let mut bht = BHTree::new(self.theta, graph_size, min_dim, min_dim, min_dim);

        for p in new_points_iter {
            bht.add_point(p);
        }

        return bht;
    }

    pub fn write_to_csv(&self, filename: String) -> Result<(), Box<dyn Error>> {
        info!("writing bht to file: {}", filename);
        let mut wtr = csv::Writer::from_path(filename.clone())?;
        wtr.write_record(&["mass", "x_pos", "y_pos", "z_pos", "x_vel", "y_vel", "z_vel"])?;

        let mut record_v: Vec<[String; 7]> = vec![];
        for p in self.root.get_points().iter() {
            let (x, y, z) = p.position();
            let mass = p.mass();
            let (xv, yv, zv) = p.velocity().position();
            record_v.push([mass, x, y, z, xv, yv, zv].map(|val| val.to_string()));
        }

        thread::spawn(move || {
            info!("flushing {}", filename.clone());
            for record in record_v.iter() {
                wtr.write_record(record).unwrap();
            }
            wtr.flush().unwrap();
            info!("done flushing {}", filename);
        });

        Ok(())
    }
}

fn should_merge(p1: Point, p2: Point) -> bool {
    let dist = p1.distance_to(p2);
    return (dist <= p1.schwarzchild_radius()) || (dist <= p2.schwarzchild_radius());
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    time: f64,
    points: Vec<Point>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BHNode {
    theta: f64,
    center_of_mass: Point,
    point: Option<Point>,
    count: i32,
    region_size: f64,
    xloc: f64,
    yloc: f64,
    zloc: f64,
    children: Vec<BHNode>,
}

impl BHNode {
    pub fn new(theta: f64, region_size: f64, x: f64, y: f64, z: f64) -> BHNode {
        trace!(
            "creating node {}, {}, {} // region size {}",
            x,
            y,
            z,
            region_size,
        );
        return BHNode {
            theta: theta,
            center_of_mass: Point::new_zero(),
            region_size: region_size,
            xloc: x,
            yloc: y,
            zloc: z,
            children: vec![],
            count: 0,
            point: None,
        };
    }

    fn center_of_mass(&self) -> Point {
        return self.center_of_mass;
    }

    fn calculate_force(&self, dt: f64, p: Point) -> Vec3d {
        if p == self.center_of_mass() || self.count == 0 {
            return Vec3d::new_zero();
        }

        let ratio = self.region_size / self.center_of_mass().distance_to(p);
        if ratio < self.theta {
            // Sufficiently far away to use this node's COM.
            return p.force_from(self.center_of_mass());
        }

        let mut force = Vec3d::new_zero();
        for child in self.children.iter() {
            force += child.calculate_force(dt, p);
        }
        return force;
    }

    // Returns the delta on the count.
    fn add_point(&mut self, p: Point) -> i32 {
        let (oldx, oldy, oldz) = self.center_of_mass.position();
        let old_mass = self.center_of_mass.mass();
        let new_mass = self.center_of_mass.mass() + p.mass();
        let (x, y, z) = p.position();
        let new_vel =
            ((self.center_of_mass.velocity() * old_mass) + (p.mass() * p.velocity())) / new_mass;
        // @tallen this is likely wrong.
        self.center_of_mass = Point::new(
            new_mass,
            (old_mass * oldx + x * p.mass()) / (new_mass),
            (old_mass * oldy + y * p.mass()) / (new_mass),
            (old_mass * oldz + z * p.mass()) / (new_mass),
            new_vel,
        );
        trace!("COM updated to {}", self.center_of_mass);

        self.count += 1;

        if self.count == 1 {
            // This is the first point to be inserted into the node, so there's nothing left to do.
            self.point = Some(p);
            return 0;
        }

        if self.count == 2 && self.children.is_empty() {
            if should_merge(self.point.unwrap(), p) {
                self.point = Some(self.center_of_mass);
                self.count -= 1;
                return -1;
            }

            self.split();
            match self.point {
                Some(local_pt) => self.add_to_child(local_pt),
                None => panic!("inconsistency in node"),
            };
            self.point = None;
        }

        self.add_to_child(p);
        self.count = 0;
        self.children.iter().for_each(|x| self.count += x.count);
        return 0;
    }

    fn add_to_child(&mut self, p: Point) -> i32 {
        // There must be children if trying to add a point to one of them.
        debug_assert!(!self.children.is_empty());

        let (x, y, z) = p.position();
        for child in self.children.iter_mut() {
            let xcontains = (child.xloc..(child.xloc + child.region_size)).contains(&x);
            let ycontains = (child.yloc..(child.yloc + child.region_size)).contains(&y);
            let zcontains = (child.zloc..(child.zloc + child.region_size)).contains(&z);
            if !xcontains || !ycontains || !zcontains {
                continue;
            }

            return child.add_point(p);
        }

        warn!(
            "point {:?} not contained in any children within range starting @ ({},{},{}) with region size {}",
            p, self.xloc,self.yloc,self.zloc,   self.region_size,
        );
        return 0;
    }

    fn split(&mut self) {
        // If we're splitting, there should not be children already.
        debug_assert!(self.children.is_empty());

        self.children.reserve(8);
        let child_region = self.region_size / 2.0;
        for x in [self.xloc, self.xloc + child_region] {
            for y in [self.yloc, self.yloc + child_region] {
                for z in [self.zloc, self.zloc + child_region] {
                    self.children
                        .push(BHNode::new(self.theta, child_region, x, y, z))
                }
            }
        }
        debug_assert_eq!(self.children.len(), 8);
    }

    fn get_points(&self) -> Vec<Point> {
        if self.children.is_empty() {
            return vec![self.point.expect("point")];
        }

        // We're dealing with a branch node.
        let mut v: Vec<Point> = Vec::new();
        for c in self.children.iter() {
            if c.count > 0 {
                v.append(&mut c.get_points());
            }
        }

        return v;
    }
}

#[cfg(test)]
mod test_bht {
    use crate::geometry::bh_tree::{BHTree, Point, Vec3d};

    #[test]
    fn starts_with_0com() {
        let bht = BHTree::new(0.5, 1e10, 0., 0., 0.);
        assert_eq!(bht.root.center_of_mass, Point::new_zero());
    }

    #[test]
    fn test_add_point() {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        for i in 1..100 {
            let mut bht = BHTree::new(1.0 / (i as f64), rng.gen_range(1.0..1337.), 0., 0., 0.);
            let pt = Point::new(1.0, 2.0, 2.0, 2.0, Vec3d::new_zero());
            bht.add_point(pt);
            assert_eq!(bht.root.xloc, 0.0);
            assert_eq!(bht.root.yloc, 0.0);
            assert_eq!(bht.root.zloc, 0.0);
            assert_eq!(bht.root.zloc, 0.0);
            assert_eq!(bht.root.center_of_mass(), pt);

            let pt2 = Point::new(1.0, 0.0, 0.0, 0.0, Vec3d::new_zero());
            bht.add_point(pt2);
            let expected = Point::new(2.0, 1., 1., 1., Vec3d::new_zero());
            assert_eq!(bht.root.center_of_mass(), expected);

            let pt3 = Point::new(2.0, 3.0, 3.0, 3.0, Vec3d::new_zero());
            bht.add_point(pt3);
            let expected = Point::new(4.0, 2., 2., 2., Vec3d::new_zero());
            assert_eq!(bht.root.center_of_mass(), expected);
        }
    }

    #[test]
    fn test_step_calculation() {
        let mut bht = BHTree::new(0.5, 5., 0., 0., 0.);
        let pt = Point::new(1e9, 2.0, 2.0, 2.0, Vec3d::new_zero());
        bht.add_point(pt);
        let pt = Point::new(1e9, 0.0, 0.0, 0.0, Vec3d::new_zero());
        bht.add_point(pt);

        for _ in 1..100 {
            bht = bht.next(1.0);
        }
    }

    #[test]
    fn serdes_test() {
        let pt = Point::new(1e9, 2.0, 2.0, 2.0, Vec3d::new_zero());
        let mut bht = BHTree::new(0.5, 5.0, 0., 0., 0.);
        let pt2 = Point::new(1e9, 1.0, 2.0, 1.0, Vec3d::new_zero());
        bht.add_point(pt);
        bht.add_point(pt2);
        let serialized = serde_json::to_string_pretty(&bht).unwrap();
        let bht2: BHTree = serde_json::from_str(&serialized).unwrap();
        let rt_serialized = serde_json::to_string_pretty(&bht2).unwrap();
        assert_eq!(serialized, rt_serialized);
    }

    #[test]
    fn merge_test() {
        let mut bht = BHTree::new(0.5, 5., 0., 0., 0.);

        assert_eq!(bht.root.count, 0);
        bht.add_point(Point::new(1., 0., 0., 0., Vec3d::new_zero()));
        assert_eq!(bht.root.count, 1);
        bht.add_point(Point::new(1., 1., 0., 0., Vec3d::new_zero()));
        assert_eq!(bht.root.count, 2);

        bht.add_point(Point::new(1e99, 1., 1., 0., Vec3d::new_zero()));
        assert_eq!(bht.root.count, 3);

        // This should merge everything into a single node.
        let next = bht.next(1.);
        assert_eq!(next.root.count, 1, "wtf");
    }
}
