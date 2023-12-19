use crate::{util::vector::Vec3, ray::{Ray, Interval}, bvh::aabb::AABB};
use super::{hittable::{Hittable, HitRecord}, material::Material};

#[derive(Clone)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    bbox: AABB,
    material: Material
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Material) -> Triangle {
        let mut x = Interval::rev();
        let mut y = Interval::rev();
        let mut z = Interval::rev();
        for vec in [a, b, c] {
            x.min = f64::min(x.min, vec.get(0));
            y.min = f64::min(y.min, vec.get(1));
            z.min = f64::min(z.min, vec.get(2));

            x.max = f64::min(x.max, vec.get(0));
            y.max = f64::min(y.max, vec.get(1));
            z.max = f64::min(z.max, vec.get(2));
        }

        Triangle {
            a,
            b,
            c,
            bbox: AABB::new(x, y, z),
            normal: (b - a).cross(c - a).normalized(),
            material
        }
    }

    pub fn mv(&mut self, delta_pos: Vec3) {
        self.a = self.a + delta_pos;
        self.b = self.b + delta_pos;
        self.c = self.c + delta_pos;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let ndd = self.normal * ray.direction;
        if ndd.abs() < 1e-6 {
            return None;
        }

        let t = -(self.normal * ray.origin - self.normal * self.a) / ndd;
        if t <= interval.min || t >= interval.max {
            return None;
        }
        let point = ray.origin + ray.direction * t;

        let edge0 = self.b - self.a;
        let edge1 = self.c - self.b;
        let edge2 = self.a - self.c;
        let c0 = point - self.a;
        let c1 = point - self.b;
        let c2 = point - self.c;
        let test0 = edge0.cross(c0);
        let test1 = edge1.cross(c1);
        let test2 = edge2.cross(c2);
        if self.normal * test0 < 0.0 || self.normal * test1 < 0.0 || self.normal * test2 < 0.0 {
            return None;
        }
        let front_face = ray.direction * self.normal < 0.0;
        Some(HitRecord {
            t,
            point,
            normal: if front_face {
                self.normal
            } else {
                self.normal * (-1.0)
            },
            material: self.material
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}