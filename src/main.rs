use log::info;
use rand::{thread_rng, Rng};

use crate::geometry::bh_tree::BHTree;
use crate::geometry::vec3d::{Point, Vec3d};

pub mod geometry;

const THETA: f64 = 0.5;
const GRAPH_SIZE: f64 = 1e6;
const NUM_POINTS: u64 = 1000;

fn main() {
    env_logger::init();
    log::set_logger(env_logger::);

    let mut rng = thread_rng();
    let mut bht = BHTree::new(THETA, GRAPH_SIZE);

    for _ in 0..NUM_POINTS {
        let x: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let y: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let z: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let p = Point::new(1.0, x, y, z, Vec3d::new_zero());
        bht.add_point(p);
    }
}
