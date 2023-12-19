use crate::util::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalized()
        }
    }
}

#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval {
            min,
            max
        }
    }

    pub fn standard() -> Interval {
        Interval {
            min: 1e-6,
            max: 1e3
        }
    }

    pub fn max() -> Interval {
        Interval {
            min: -1e6,
            max:  1e6
        }
    }

    pub fn rev() -> Interval {
        Interval {
            min:  1e6,
            max: -1e6
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        Interval {
            min: self.min - delta / 2.0,
            max: self.max + delta / 2.0
        }
    }
}