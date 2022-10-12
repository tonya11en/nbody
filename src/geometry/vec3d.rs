use serde::{Deserialize, Serialize};
use std::fmt::Display;

const G: f64 = 6.67430e-11;
const C: f64 = 299792458.0;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}*({}, {}, {})",
            self.magnitude(),
            self.x,
            self.y,
            self.z
        );
    }
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        return Vec3d { x: x, y: y, z: z };
    }

    pub fn new_zero() -> Vec3d {
        return Vec3d::new(0.0, 0.0, 0.0);
    }

    pub fn magnitude(&self) -> f64 {
        let (x, y, z) = self.position();
        return (x * x + y * y + z * z).sqrt();
    }

    pub fn position(self) -> (f64, f64, f64) {
        return (self.x, self.y, self.z);
    }

    pub fn distance(self, other: Vec3d) -> Vec3d {
        let (ox, oy, oz) = other.position();
        return Vec3d::new(ox - self.x, oy - self.y, oz - self.z);
    }
}

impl std::ops::AddAssign<Vec3d> for Vec3d {
    fn add_assign(&mut self, rhs: Vec3d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Vec3d) -> Self::Output {
        return Vec3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl std::ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, s: f64) -> Self::Output {
        let (x, y, z) = self.position();
        return Vec3d::new(s * x, s * y, s * z);
    }
}

impl std::ops::Mul<Vec3d> for f64 {
    type Output = Vec3d;

    fn mul(self, s: Vec3d) -> Self::Output {
        let (x, y, z) = s.position();
        return Vec3d::new(self * x, self * y, self * z);
    }
}

impl std::ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, s: f64) -> Self::Output {
        let (x, y, z) = self.position();
        return Vec3d::new(x / s, y / s, z / s);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    mass: f64,
    vel: Vec3d,
    schwarzchild_radius: f64,

    x: f64,
    y: f64,
    z: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} @ ({}, {}, {})", self.vel, self.x, self.y, self.z);
    }
}

impl Point {
    pub fn new(mass: f64, x: f64, y: f64, z: f64, velocity: Vec3d) -> Point {
        return Point {
            mass: mass,
            vel: velocity,
            x: x,
            y: y,
            z: z,
            schwarzchild_radius: 2.0 * G * mass / (C * C),
        };
    }

    pub fn new_zero() -> Point {
        return Point::new(0., 0., 0., 0., Vec3d::new_zero());
    }

    pub fn schwarzchild_radius(&self) -> f64 {
        return self.schwarzchild_radius;
    }

    pub fn mass(&self) -> f64 {
        return self.mass;
    }

    pub fn velocity(&self) -> Vec3d {
        return self.vel;
    }

    pub fn apply_force(self, dt: f64, force: Vec3d) -> Point {
        let a = force / self.mass * dt;
        let v = self.vel + (a * dt);
        let (vx, vy, vz) = v.position();
        return Point::new(
            self.mass,
            self.x + vx * dt,
            self.y + vy * dt,
            self.z + vz * dt,
            v,
        );
    }

    pub fn force_from(self, p: Point) -> Vec3d {
        let dist = self.distance_to(p);
        let mass: f64 = self.mass;
        let f = G * mass * p.mass() / (dist * dist);
        let x = (p.x - self.x) / dist * f;
        let y = (p.y - self.y) / dist * f;
        let z = (p.z - self.z) / dist * f;
        return Vec3d::new(x, y, z);
    }

    pub fn position(self) -> (f64, f64, f64) {
        return (self.x, self.y, self.z);
    }

    pub fn distance_to(self, other: Point) -> f64 {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;
        return (x * x + y * y + z * z).sqrt();
    }
}

#[cfg(test)]
mod test {
    use crate::{Point, Vec3d};

    #[test]
    fn test_distance() {
        let origin = Point::new(1., 0., 0., 0., Vec3d::new_zero());
        let p1 = Point::new(1., -1., -2., -2., Vec3d::new(1., 2., 3.));
        let p2 = Point::new(1., 1., 2., 2., Vec3d::new(1., 2., 3.));

        assert_eq!(origin.distance_to(p2), 3.0);
        assert_eq!(p1.distance_to(p2), 6.0);
    }

    #[test]
    fn test_force() {
        let origin = Point::new(1., 0., 0., 0., Vec3d::new_zero());
        let force = Vec3d::new(1.0, 0., 0.);
        let new_pt = origin.apply_force(1.0, force);
        let (x, y, z) = new_pt.position();
        assert_eq!(x, 1.0);
        assert_eq!(y, 0.0);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let m = v.magnitude();
        assert_eq!(m, (14_f64).sqrt());
    }

    #[test]
    fn test_vec_operations() {
        // Multiplication.
        let v = Vec3d::new(1.0, 2.0, 3.0);
        let v2 = v * 2.0;
        let (x, y, z) = v2.position();
        assert_eq!(x, 2.0);
        assert_eq!(y, 4.0);
        assert_eq!(z, 6.0);
        let v3 = 2.0 * v;
        let (x, y, z) = v3.position();
        assert_eq!(x, 2.0);
        assert_eq!(y, 4.0);
        assert_eq!(z, 6.0);

        // Division.
        let v4 = v3 / 2.0;
        let (x, y, z) = v4.position();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);

        // Add.
        let vs = Vec3d::new(-1.0, 0.0, 2.0);
        let vt = v + vs;
        let (x, y, z) = vt.position();
        assert_eq!(x, 0.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 5.0);
    }

    #[test]
    fn test_close_attraction() {
        let p1 = Point::new(1e30, 2.0, 2.0, 2.0, Vec3d::new_zero());
        let p2 = Point::new(1e30, 0.0, 0.0, 0.0, Vec3d::new_zero());
        let p3 = Point::new(1e30, 1.0, 1.0, 1.0, Vec3d::new_zero());
        dbg!(p1.force_from(p2));
        dbg!(p1.force_from(p3));
    }
}
