use std::error::Error;

use log::info;
use rand::{thread_rng, Rng};

use crate::geometry::bh_tree::BHTree;
use crate::geometry::vec3d::{Point, Vec3d};

pub mod geometry;

const THETA: f64 = 0.4;
const GRAPH_SIZE: f64 = 10000.;
const NUM_POINTS: u64 = 10000;
const TIME_STEP: f64 = 1.;
const STEPS: i32 = 10000;
const PARTICLE_MASS: f64 = 5e10;

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
    let mut bht = BHTree::new(
        THETA,
        2. * GRAPH_SIZE,
        -GRAPH_SIZE,
        -GRAPH_SIZE,
        -GRAPH_SIZE,
    );

    for _ in 0..NUM_POINTS {
        let theta = rng.gen_range(0.0..(2.0 * (std::f64::consts::PI)));
        let phi = rng.gen_range(0.0..(2.0 * (std::f64::consts::PI)));
        let r = rng.gen_range(0.0..GRAPH_SIZE);

        let x: f64 = r * theta.sin() * phi.cos();
        let y: f64 = r * theta.sin() * phi.sin();
        let z: f64 = r * (theta.cos());

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
