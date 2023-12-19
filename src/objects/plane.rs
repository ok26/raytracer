use super::{hittable::{HitRecord, Hittable}, material::Material};
use crate::{ray::{Ray, Interval}, util::vector::Vec3, bvh::aabb::AABB};

#[derive(Clone)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f64,
    bbox: AABB,
    material: Material
}

impl Plane {
    pub fn new(normal: Vec3, point: Vec3, material: Material) -> Plane {
        let normal = normal.normalized();
        let distance = point.project(normal).length();
        Plane {
            normal,
            distance,
            bbox: AABB::new(Interval::max(), Interval::max(), Interval::max()),
            material
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if (ray.direction * self.normal).abs() <= 1e-6  {
            return None;
        }
        
        let t = (self.distance - (ray.origin * self.normal)) / (ray.direction * self.normal);
        if t <= interval.min || t >= interval.max {
            return None;
        }

        let front_face = ray.direction * self.normal < 0.0;

        Some(HitRecord {
            t,
            point: ray.origin + (ray.direction * t),
            normal: if front_face {
                self.normal
            } else {
                self.normal * -1.0
            },
            material: self.material
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}