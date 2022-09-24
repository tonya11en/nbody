#[derive(Debug)]
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

    pub fn add(&self, b: Vec3d) -> Vec3d {
        return Vec3d::new(self.x + b.x, self.y + b.y, self.z + b.z);
    }

    pub fn magnitude(&self) -> f64 {
        return self.mag;
    }

    pub fn scalar_divide(&self, s: f64) -> Vec3d {
        let (x, y, z) = self.position();
        return Vec3d::new(x / s, y / s, z / s);
    }

    pub fn scalar_multiply(&self, s: f64) -> Vec3d {
        let (x, y, z) = self.position();
        return Vec3d::new(s * x, s * y, s * z);
    }

    fn position(&self) -> (f64, f64, f64) {
        return (self.mag * self.x, self.mag * self.y, self.mag * self.z);
    }

    fn distance(&self, other: Vec3d) -> Vec3d {
        let (ox, oy, oz) = other.position();
        return Vec3d::new(ox - self.x, oy - self.y, oz - self.z);
    }
}

fn norm(x: f64, y: f64, z: f64) -> f64 {
    return (x * x + y * y + z * z).sqrt();
}

#[derive(Debug)]
pub struct Node {
    mass: f64,
    pos: Vec3d,
    vel: Vec3d,
}

impl Node {
    pub fn new(mass: f64, position: Vec3d, velocity: Vec3d) -> Node {
        return Node {
            mass: mass,
            pos: position,
            vel: velocity,
        };
    }

    pub fn mass(&self) -> f64 {
        return self.mass;
    }

    pub fn apply_force(&self, dt: f64, force: Vec3d) -> Node {
        let a = force.scalar_divide(self.mass).scalar_multiply(dt);
        let v = self.vel.add(a).scalar_multiply(dt);
        let p = self.pos.add(v);
        return Node::new(self.mass, p, v);
    }

    fn position(&self) -> Vec3d {
        return self.pos;
    }

    fn distance(&self, other: Vec3d) -> Vec3d {
        return self.pos.distance(other);
    }
}
