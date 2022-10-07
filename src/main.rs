use csv::WriterBuilder;
use log::{info, LevelFilter};
use rand::{thread_rng, Rng};

use crate::geometry::bh_tree::BHTree;
use crate::geometry::db::DbHandle;
use crate::geometry::vec3d::{Point, Vec3d};

pub mod geometry;

const THETA: f64 = 0.5;
const GRAPH_SIZE: f64 = 1e6;
const NUM_POINTS: u64 = 1000000;
const TIME_STEP: f64 = 1.1;
const SIM_TIME: f64 = 10.0;

fn main() -> Result<(), sled::Error> {
    env_logger::init();
    info!(
        theta = THETA, 
        graph_size = GRAPH_SIZE, 
        dt = TIME_STEP,
        sim_time = SIM_TIME,
        num_points = NUM_POINTS; 
        "starting nbody simulation");

    let mut dbh = DbHandle::new(String::from("my_db"))?;
    let mut rng = thread_rng();
    let mut bht = BHTree::new(THETA, GRAPH_SIZE);

    for _ in 0..NUM_POINTS {
        let x: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let y: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let z: f64 = rng.gen_range(0.0..GRAPH_SIZE);
        let p = Point::new(1.0, x, y, z, Vec3d::new_zero());
        bht.add_point(p);
    }

    let mut t: f64 = 0.;
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_path(String::from("/tmp/data.csv"))
        .unwrap();
    while t < SIM_TIME {
        dbh.persist(t, bht);
        t += TIME_STEP;
        bht = bht.next(TIME_STEP);
    }

    return Ok(());
}
