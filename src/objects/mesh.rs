use crate::{util::{vector::Vec3, parser::{parse_obj, parse_vex}}, ray::{Ray, Interval}, bvh::{bvh::BVHNode, aabb::AABB}};
use super::{triangle::Triangle, material::Material, hittable::{Hittable, HitRecord, HittableList}};

#[derive(Clone)]
pub struct Mesh {
    triangles: BVHNode,
    //triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        let mut list = HittableList::new(vec![]);
        for triangle in triangles.clone() {
            list.add(Box::new(triangle));
        }
        Mesh {
            triangles: BVHNode::new(&list),
            //triangles
        }
    }

    pub fn from_obj(data: &str, position: Vec3, material: Material) -> Mesh {
        let mut triangles = parse_obj(data);
        for triangle in &mut triangles {
            triangle.mv(position);
            triangle.set_material(material);
        }
        Mesh::new(triangles)
    }

    pub fn from_vex(data: &str, position: Vec3, material: Material) -> Mesh {
        let mut triangles = parse_vex(data);
        for triangle in &mut triangles {
            triangle.mv(position);
            triangle.set_material(material);
        }
        Mesh::new(triangles)
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        self.triangles.hit(ray, interval)
        /* 
        let mut closest_hit: Option<HitRecord> = None;
        for triangle in self.triangles.clone() {
            if let Some(hit_info) = triangle.hit(ray, interval) {
                if closest_hit.is_none() || closest_hit.unwrap().t > hit_info.t {
                    closest_hit = Some(hit_info);
                }
            }
        }
        closest_hit
        */
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(Interval::max(), Interval::max(), Interval::max())
    }
}