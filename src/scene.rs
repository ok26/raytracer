use crate::image::color;
use crate::objects::hittable::{Hittable, HitRecord};
use crate::camera::Camera;
use crate::image::{image::Image, color::Color};
use crate::ray::{Ray, Interval};
use crate::util::random::Random;
use crate::util::vector;

pub struct Scene<'a> {
    objects: Vec<&'a dyn Hittable>,
    camera: Camera,
    sun_strength: f64
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera, sun_strength: f64) -> Scene<'a> {
        Scene {
            objects: vec![],
            camera,
            sun_strength
        }
    }

    pub fn add_object(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }

    fn trace_ray(&mut self, ray: Ray, bounce_count: usize, rng: &mut Random) -> Color {
        let mut ray = ray;
        let mut ray_color = Color::white();
        let mut incoming_light = Color::blank();

        for _ in 0..bounce_count {

            if let Some(hit_info) = self.closest_hit(&ray) {

                let diffuse_dir = (hit_info.normal + rng.random_dir()).normalized();
                let specular_dir = ray.direction.reflect(hit_info.normal);
                ray.direction = vector::lerp(diffuse_dir, specular_dir, hit_info.material.smoothness).normalized();
                ray.origin = hit_info.point;

                let emitted_light = hit_info.material.emission_color * hit_info.material.emission_strength;
                incoming_light = incoming_light + emitted_light * ray_color;
                ray_color = ray_color * hit_info.material.color;
            }
            else {
                let t = 0.5 * (ray.direction.get(1) + 1.0);
                incoming_light = incoming_light + 
                    ray_color * color::lerp(Color::new(0.5, 0.7, 1.0), Color::white(), t) * self.sun_strength;
                break;
            }
        }
        incoming_light
    }

    fn closest_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut interval = Interval::standard();
        for i in 0..self.objects.len() {
            if closest_hit.is_some() {
                interval.max = closest_hit.unwrap().t;
            }
            if let Some(hit_info) = self.objects[i].hit(&ray, &interval) {
                if closest_hit.is_none() || hit_info.t < closest_hit.unwrap().t {
                    closest_hit = Some(hit_info);
                }
            }
        }
        closest_hit
    }

    pub fn render(&mut self, camera: &Camera, bounce_count: usize, ray_count: usize) -> Image {
        let mut image = Image::new(camera.image_width(), camera.image_height());
        for y in 0..camera.image_height() {
            println!("{}", y);
            for x in 0..camera.image_width() {

                let mut average_color = Color::blank();
                let mut rng = Random::new((x * y + x) as u32);

                for _ in 0..ray_count {

                    let viewport_pixel = self.camera.viewport_upper_left() 
                        + self.camera.delta_u() * ((x as f64) + rng.next()) 
                        + self.camera.delta_v() * ((y as f64) + rng.next());
                    let ray_direction = viewport_pixel - self.camera.position();

                    let ray = Ray::new(
                        self.camera.position(),
                        ray_direction
                    );

                    average_color = average_color + self.trace_ray(ray, bounce_count, &mut rng);
                }
                image.set(x, y, color::linear_to_gamma(average_color / ray_count as f64));
            }
            if y % 100 == 0 {
                println!("{}", y);
            }
        }
        image
    }
}