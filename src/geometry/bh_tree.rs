use crate::{Point, Vec3d};

pub struct BHTree {
    root: BHNode,
    theta: f64,
    graph_size: f64,
    points: Vec<Point>,
}

impl BHTree {
    pub fn new(theta: f64, graph_size: f64) -> BHTree {
        let root = BHNode::new(theta, graph_size, 0., 0., 0.);
        return BHTree {
            root: root,
            theta: theta,
            graph_size: graph_size,
            points: Vec::new(),
        };
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
        self.root.add_point(p);
    }

    fn next(&self, dt: f64) -> BHTree {
        let mut bht = BHTree::new(self.theta, self.graph_size);
        for p in self.points.iter() {
            let force = self.root.calculate_force(dt, *p);
            let new_p = p.apply_force(dt, force);
            bht.add_point(new_p);
        }
        return bht;
    }
}

pub struct BHNode {
    theta: f64,
    center_of_mass: Point,
    point: Option<Point>,
    count: usize,
    region_size: f64,
    xloc: f64,
    yloc: f64,
    zloc: f64,
    children: Vec<BHNode>,
}

impl BHNode {
    pub fn new(theta: f64, region_size: f64, x: f64, y: f64, z: f64) -> BHNode {
        let r = region_size / 2.;
        let vel = Vec3d::new_zero();
        let com = Point::new(0., 0., 0., 0., vel);
        return BHNode {
            theta: theta,
            center_of_mass: com,
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
        assert!(self.count > 0);
        return self.center_of_mass;
    }

    fn calculate_force(&self, dt: f64, p: Point) -> Vec3d {
        if p == self.center_of_mass() || self.count == 0 {
            return Vec3d::new_zero();
        }

        let ratio = self.region_size / self.center_of_mass().distance_to(p);
        if ratio < self.theta {
            // Sufficiently far away to use this node's COM.
            return p.force_from(dt, p);
        }

        let mut force = Vec3d::new_zero();
        for child in self.children.iter() {
            force += child.calculate_force(dt, p);
        }
        return force;
    }

    fn add_point(&mut self, p: Point) {
        let (oldx, oldy, oldz) = self.center_of_mass.position();
        let new_mass = self.center_of_mass.mass() + p.mass();
        let (x, y, z) = p.position();
        let count = self.count as f64;
        self.center_of_mass = Point::new(
            new_mass,
            (oldx * count + x) / (count + 1.),
            (oldy * count + y) / (count + 1.),
            (oldz * count + z) / (count + 1.),
            Vec3d::new_zero(),
        );

        self.count += 1;
        if self.count == 1 {
            dbg!(
                p.position(),
                self.region_size,
                self.xloc,
                self.yloc,
                self.zloc,
            );
            // This is the first point to be inserted into the node, so there's nothing left to do.
            self.point = Some(p);
            return;
        } else if self.count == 2 {
            self.split();
            match self.point {
                Some(p) => self.add_to_child(p),
                None => panic!("node should have a point residing within"),
            };
            self.point = None;
        }

        self.add_to_child(p);
        dbg!(self.validate());
    }

    fn validate(&self) {
        if self.children.is_empty() {
            let c1 = self.count == 0 && self.point == None;
            let c2 = self.count == 1;
            assert!(c1 || c2);
        } else {
            assert_eq!(self.point, None);
            for child in self.children.iter() {
                child.validate();
            }
        }
    }

    fn add_to_child(&mut self, p: Point) {
        // There must be children if trying to add a point to one of them.
        debug_assert!(!self.children.is_empty());

        let (x, y, z) = p.position();
        for child in self.children.iter_mut() {
            if !(child.xloc..(child.xloc + child.region_size)).contains(&x)
                || !(child.yloc..(child.yloc + child.region_size)).contains(&y)
                || !(child.zloc..(child.zloc + child.region_size)).contains(&z)
            {
                continue;
            }

            child.add_point(p);
            return;
        }

        panic!(
            "point {:?} not contained in any children within range starting @ ({},{},{}) with region size {}",
            p, self.xloc,self.yloc,self.zloc,   self.region_size,
        );
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
}

#[cfg(test)]
mod test_bht {
    use crate::geometry::bh_tree::{BHTree, Point, Vec3d};

    #[test]
    fn test_add_point() {
        let mut bht = BHTree::new(0.5, 1e10);
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

        let pt3 = Point::new(1.0, 2.0, 2.0, 2.1, Vec3d::new_zero());
        bht.add_point(pt3);
        let expected = Point::new(3.0, 0., 0., 0., Vec3d::new_zero());
        assert_eq!(bht.root.center_of_mass(), expected);
    }

    #[test]
    fn test_step_calculation() {
        let mut bht = BHTree::new(0.5, 1e10);
        let pt = Point::new(1.0, 2.0, 2.0, 2.0, Vec3d::new_zero());
        bht.add_point(pt);
        let pt = Point::new(1.0, 0.0, 0.0, 0.0, Vec3d::new_zero());
        bht.add_point(pt);

        let next = bht.next(1.0);
        next.theta;
    }
}
