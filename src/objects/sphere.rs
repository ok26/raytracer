use super::{hittable::{HitRecord, Hittable}, material::Material};
use crate::{ray::{Ray, Interval}, util::vector::Vec3, bvh::aabb::AABB};

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    bbox: AABB,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        let x = Interval::new(center.get(0) - radius, center.get(0) + radius);
        let y = Interval::new(center.get(1) - radius, center.get(1) + radius);
        let z = Interval::new(center.get(2) - radius, center.get(2) + radius);

        Sphere { 
            center, 
            radius,
            bbox: AABB::new(x, y, z),
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let v = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let b = (v * 2.0) * ray.direction;
        let c = v * v - self.radius * self.radius;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }

        let t = (-b - disc.sqrt()) / (2.0 * a);
        if t <= interval.min || t >= interval.max {
            return None;
        }

        let point = ray.origin + (ray.direction * t);
        let normal = (point - self.center) * (1.0 / self.radius);
        let front_face = (ray.direction * normal) < 0.0;

        Some(HitRecord {
            t,
            point,
            normal: if front_face {
                normal.normalized()
            } else {
                normal.normalized() * (-1.0)
            },
            material: self.material
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}