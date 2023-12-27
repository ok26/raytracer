use crate::{ray::{Interval, Ray}, util::vector::Vec3};

#[derive(Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB { x, y, z }
    }

    pub fn standard() -> AABB {
        AABB {
            x: Interval::max(),
            y: Interval::max(),
            z: Interval::max()
        }
    }

    pub fn comb(a: &AABB, b: &AABB) -> AABB {
        AABB {
            x: Interval::new(f64::min(a.x.min, b.x.min), f64::max(a.x.max, b.x.max)),
            y: Interval::new(f64::min(a.y.min, b.y.min), f64::max(a.y.max, b.y.max)),
            z: Interval::new(f64::min(a.z.min, b.z.min), f64::max(a.z.max, b.z.max))
        }
    }

    pub fn from_vec(a: &Vec3, b: &Vec3) -> AABB {
        AABB {
            x: Interval::new(f64::min(a.get(0), b.get(0)), f64::max(a.get(0), b.get(0))),
            y: Interval::new(f64::min(a.get(1), b.get(1)), f64::max(a.get(1), b.get(1))),
            z: Interval::new(f64::min(a.get(2), b.get(2)), f64::max(a.get(2), b.get(2)))
        }
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis")
        }
    }

    pub fn hit(&self, ray: &Ray, interval: &Interval) -> bool {

        let mut t_min = interval.min;
        let mut t_max = interval.max;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction.get(a);
            let origin = ray.origin.get(a);
            let min = self.axis(a).min;
            let max = self.axis(a).max;

            let mut t0 = (min - origin) * inv_d;
            let mut t1 = (max - origin) * inv_d;

            if inv_d < 0.0{
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}