use std::error::Error;

use log::info;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

use crate::geometry::bh_tree::BHTree;
use crate::geometry::vec3d::{Point, Vec3d};

pub mod geometry;

const THETA: f64 = 0.5;
const GRAPH_SIZE: f64 = 10000.;
const NUM_POINTS: u64 = 50000;
const TIME_STEP: f64 = 1.0;
const STEPS: i32 = 10000;
const PARTICLE_MASS_MEAN: f64 = 5e10;
const PARTICLE_MASS_STDDEV: f64 = 5e10;

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

    let normal = Normal::new(PARTICLE_MASS_MEAN, PARTICLE_MASS_STDDEV).unwrap();

    for _ in 0..NUM_POINTS {
        let mut x: f64 = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        let mut y: f64 = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        let mut z: f64 = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        while (x * x + y * y + z * z).sqrt() > GRAPH_SIZE {
            x = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
            y = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
            z = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        }

        let mass = normal.sample(&mut rand::thread_rng());
        let p = Point::new(mass, x, y, z, Vec3d::new_zero());
        bht.add_point(p);
    }

    for t in 0..STEPS {
        let filepath = String::from(format!("output/out-{}.csv", t));
        bht.write_to_csv(filepath)?;
        bht = bht.next(TIME_STEP);
    }

    return Ok(());
}
