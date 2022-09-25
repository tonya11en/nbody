#[derive(Debug, Copy, Clone)]
pub struct Vec3d {
    // Unit vec.
    x: f64,
    y: f64,
    z: f64,

    // Magnitude of the vector.
    mag: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        let mag = (x * x + y * y + z * z).sqrt();
        return Vec3d {
            x: x / mag,
            y: y / mag,
            z: z / mag,
            mag: mag,
        };
    }

    pub fn magnitude(&self) -> f64 {
        return self.mag;
    }

    pub fn position(self) -> (f64, f64, f64) {
        return (self.mag * self.x, self.mag * self.y, self.mag * self.z);
    }

    pub fn distance(self, other: Vec3d) -> Vec3d {
        let (ox, oy, oz) = other.position();
        return Vec3d::new(ox - self.x, oy - self.y, oz - self.z);
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

fn norm(x: f64, y: f64, z: f64) -> f64 {
    return (x * x + y * y + z * z).sqrt();
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    mass: f64,
    vel: Vec3d,

    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(mass: f64, x: f64, y: f64, z: f64, velocity: Vec3d) -> Point {
        return Point {
            mass: mass,
            vel: velocity,
            x: x,
            y: y,
            z: z,
        };
    }

    pub fn mass(&self) -> f64 {
        return self.mass;
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
