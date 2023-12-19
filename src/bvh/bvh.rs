use std::time::SystemTime;
use crate::{objects::hittable::{Hittable, HittableList, HitRecord}, util::random::Random, ray::{Ray, Interval}};
use super::aabb::AABB;

#[derive(Clone)]
pub struct BVHNode {
    bbox: Option<AABB>,
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>
}

impl BVHNode {
    pub fn new(list: &HittableList) -> BVHNode {
        let mut rng = Random::new(SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32);
        let axis = rng.randint(0, 3);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => panic!("Invalid axis")
        };
        
        match list.objects.len() {
            1 => {
                let left = list.objects[0].clone();
                let right = list.objects[0].clone();

                BVHNode {
                    bbox: list.bbox.clone(),
                    left,
                    right,
                }
            },
            2 => {
                let left = list.objects[0].clone();
                let right = list.objects[1].clone();

                BVHNode {
                    bbox: Some(AABB::comb(&left.bounding_box(), &right.bounding_box())),
                    left,
                    right,
                }
            },
            _ => {
                let mut objects = list.objects.clone();
                objects.sort_by(comparator);
                let left = BVHNode::new(&HittableList::new(objects[..(objects.len() / 2)].to_vec()));
                let right = BVHNode::new(&HittableList::new(objects[(objects.len() / 2)..].to_vec()));
                BVHNode {
                    bbox: Some(AABB::comb(&left.bounding_box(), &right.bounding_box())),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        a.bounding_box().axis(axis).min.partial_cmp(&b.bounding_box().axis(axis).max).unwrap()
    }

    fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if !self.bbox.as_ref().unwrap().hit(ray, interval) {
            return None;
        }
        let hit_left = self.left.hit(ray, interval);
        let hit_right = self.right.hit(ray, interval);

        if hit_left.is_some() && hit_right.is_some() {
            let hit_left = hit_left.unwrap();
            let hit_right = hit_right.unwrap();
            if hit_left.t < hit_right.t {
                return Some(hit_left);
            }
            else {
                return Some(hit_right);
            }
        }
        hit_left.or(hit_right)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone().unwrap()
    }
}