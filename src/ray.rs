use super::{Vector3,Point3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    // Getters
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    // Setters
    pub fn set_origin(&mut self, origin: Point3) {
        self.origin = origin;
    }

    pub fn set_direction(&mut self, direction: Vector3) {
        self.direction = direction;
    }

    // Methods
    pub fn at(&self, t: f64) -> Point3 {
        *self.origin() + t * *self.direction()
    }
}