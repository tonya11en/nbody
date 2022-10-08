use std::error::Error;

use log::info;
use rand::{thread_rng, Rng};

use crate::geometry::bh_tree::BHTree;
use crate::geometry::vec3d::{Point, Vec3d};

pub mod geometry;

const THETA: f64 = 0.5;
const GRAPH_SIZE: f64 = 1496e11;
const NUM_POINTS: u64 = 10000;
const TIME_STEP: f64 = 3600.;
const STEPS: i32 = 1000;
const PARTICLE_MASS: f64 = 1e32;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!(
        theta = THETA, 
        graph_size = GRAPH_SIZE, 
        dt = TIME_STEP,
        steps = STEPS,
        num_points = NUM_POINTS; 
        "starting nbody simulation");

    let mut rng = thread_rng();
    let mut bht = BHTree::new(THETA, GRAPH_SIZE, 0., 0., 0.);

    for _ in 0..NUM_POINTS {
        let x: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let y: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let z: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let p = Point::new(PARTICLE_MASS, x, y, z, Vec3d::new_zero());
        bht.add_point(p);
    }

    for t in 0..STEPS {
        let filepath = String::from(format!("output/out-{}.csv", t));
        bht.write_to_csv(filepath)?;
        bht = bht.next(TIME_STEP);
    }

    return Ok(());
}
