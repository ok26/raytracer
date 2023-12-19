use crate::bvh::aabb::AABB;
use crate::ray::{Ray, Interval};
use crate::util::vector::Vec3;
use super::material::Material;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material
}

pub trait Hittable: Sync + Send + HittableClone {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}
impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct HittableList {
    pub bbox: Option<AABB>,
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        let mut bbox = None;

        for object in &objects {
            if bbox.is_none() {
                bbox = Some(object.bounding_box());
            }
            else {
                bbox = Some(AABB::comb(&bbox.unwrap(), &object.bounding_box()));
            }
        }
        HittableList { bbox, objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        if self.bbox.is_none() {
            self.bbox = Some(object.bounding_box());
        }
        else {
            self.bbox = Some(AABB::comb(&self.bbox.clone().unwrap(), &object.bounding_box()));
        }
        self.objects.push(object);
    }
}