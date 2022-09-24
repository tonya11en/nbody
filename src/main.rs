use crate::geometry::vec3d::{Node, Vec3d};

pub mod geometry;

const G: f64 = 6.67430e-11;

fn main() {
    let n = Node {
        mass: 69.0,
        pos: Vec3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        vel: Vec3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    };

    println!("{:?}", n);
}
