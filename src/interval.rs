use core::f64;

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: -f64::INFINITY };
    pub const UNIVERSE: Interval = Interval { min: -f64::INFINITY, max: f64::INFINITY };

    pub fn new() -> Self {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn build(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }

        if x > self.max {
            return self.max;
        }

        x
    }


}
