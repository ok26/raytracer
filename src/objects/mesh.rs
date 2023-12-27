use crate::{util::{vector::Vec3, parser::{parse_obj, parse_vex}}, ray::{Ray, Interval}, bvh::{bvh::BVHNode, aabb::AABB}};
use super::{triangle::Triangle, material::Material, hittable::{Hittable, HitRecord, HittableList}};

#[derive(Clone)]
pub struct Mesh {
    triangles: BVHNode,
    bbox: AABB
}

impl Mesh {
    pub fn new(
        triangles: Vec<Triangle>,
        position: Vec3,
        rotation: Vec3,
        scale: f64,
        material: Material) -> Mesh {

        let mut triangles = triangles;
        for triangle in &mut triangles {
            triangle.scale(scale);
            triangle.rotate(rotation);
            triangle.mv(position);
            triangle.set_material(material);
        }

        let mut list = HittableList::new(vec![]);
        for triangle in triangles.clone() {
            list.add(Box::new(triangle));
        }
        let node = BVHNode::new(&list);
        Mesh {
            triangles: node.clone(),
            bbox: node.bounding_box().clone()
        }
    }

    pub fn from_obj(
        data: &str, 
        position: Vec3, 
        rotation: Vec3,
        scale: f64,
        material: Material) -> Mesh {
        
        Mesh::new(parse_obj(data), position, rotation, scale, material)
    }

    pub fn from_vex(
        data: &str, 
        position: Vec3, 
        rotation: Vec3,
        scale: f64,
        material: Material) -> Mesh {

        Mesh::new(parse_vex(data), position, rotation, scale, material)
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        self.triangles.hit(ray, interval)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}