pub mod operators;
pub mod util;

use rand::Rng;
use util::dot;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub e: [f64; 3],
}

pub type Point3 = Vector3;
pub type Colour = Vector3;

impl Vector3 {
    // Builders
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { e: [x, y, z] }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_limit(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Self::random_limit(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vector3) -> Self {
        let on_unit_sphere = Self::random_unit();
        if dot(&on_unit_sphere, &normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    // Getters
    pub fn x(&self) -> &f64 {
        &self.e[0]
    }

    pub fn y(&self) -> &f64 {
        &self.e[1]
    }

    pub fn z(&self) -> &f64 {
        &self.e[2]
    }

    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.e[0] = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.e[1] = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.e[2] = z;
    }

    // Methods
    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Colour {
    pub fn new_colour(r: f64, g: f64, b: f64) -> Self {
        Colour { e: [r, g, b] }
    }

    // Getters
    pub fn r(&self) -> &f64 {
        &self.e[0]
    }

    pub fn g(&self) -> &f64 {
        &self.e[1]
    }

    pub fn b(&self) -> &f64 {
        &self.e[2]
    }

    // Setters
    pub fn set_r(&mut self, r: f64) {
        self.e[0] = r;
    }

    pub fn set_g(&mut self, g: f64) {
        self.e[1] = g;
    }

    pub fn set_b(&mut self, b: f64) {
        self.e[2] = b;
    }
}

// Traits
impl Default for Vector3 {
    fn default() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x(), self.y(), self.z())
    }
}
