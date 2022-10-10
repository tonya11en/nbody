use std::error::Error;

use log::info;
use rand::{thread_rng, Rng};

use crate::geometry::bh_tree::BHTree;
use crate::geometry::vec3d::{Point, Vec3d};

pub mod geometry;

const THETA: f64 = 0.5;
const GRAPH_SIZE: f64 = 1000.;
const NUM_POINTS: u64 = 50000;
const TIME_STEP: f64 = 1.05;
const STEPS: i32 = 100;
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
        let mut x: f64 = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        let mut y: f64 = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        let mut z: f64 = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        while (x * x + y * y + z * z).sqrt() > GRAPH_SIZE {
            x = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
            y = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
            z = rng.gen_range(-GRAPH_SIZE..GRAPH_SIZE);
        }

        let mass_delta: f64 = rng.gen_range(0.0..(10. * PARTICLE_MASS));

        let vx = y.cbrt();
        let vy = -x.cbrt();
        let vz = rng.gen_range(-1.0..1.);

        let p = Point::new(PARTICLE_MASS + mass_delta, x, y, z, Vec3d::new(vx, vy, vz));
        bht.add_point(p);
    }

    /*
    bht.add_point(Point::new(
        PARTICLE_MASS * 100.,
        0.,
        0.,
        0.,
        Vec3d::new(
            100. * rng.gen_range(-1.0..1.),
            100. * rng.gen_range(-1.0..1.),
            100. * rng.gen_range(-1.0..1.),
        ),
    ));
    */

    for t in 0..STEPS {
        let filepath = String::from(format!("output/out-{}.csv", t));
        bht.write_to_csv(filepath)?;
        bht = bht.next(TIME_STEP);
    }

    return Ok(());
}
