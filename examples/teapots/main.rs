use raytracer::{
    camera::Camera,
    scene::Scene,
    util::{vector::Vec3, random::Random},
    image::color::Color,
    objects::{mesh::Mesh, material::Material, plane::Plane, sphere::Sphere}
};

const IMAGE_WIDTH: usize = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const FOV: f64 = 60.0;
const FOCAL_LENGTH: f64 = 2.0;

const MAX_BOUNCE_COUNT: usize = 15;
const RAY_COUNT: usize = 100;

const SUN_STRENGTH: f64 = 0.5;

fn main() {
    let camera = Camera::new(
        Vec3::new([0.0, 20.0, 7.0]),
        Vec3::new([0.0, 1.0, -8.0]),
        IMAGE_WIDTH,
        FOV,
        FOCAL_LENGTH,
        ASPECT_RATIO,
    );
    
    let mut scene = Scene::new(camera.clone(), SUN_STRENGTH);

    let teapot_data = include_str!("teapot.obj");
    let mut rng = Random::new(9786237);

    for _ in 0..20 {
        scene.add_object(Box::new(Mesh::from_obj(
            teapot_data,
            Vec3::new([rng.next() * 30.0 - 15.0, rng.next() * 8.0, -rng.next() * 20.0 + 0.0]),
            Vec3::new([rng.next() * 180.0 + 90.0, rng.next() * 180.0 + 90.0, rng.next() * 180.0 + 90.0]),
            1.0,
            Material::new(
                Color::new(0.7, 0.7, 0.7),
                Color::blank(),
                0.0,
                0.3
        ))));
    }

    scene.add_object(Box::new(Plane::new(
        Vec3::new([0.0, -1.0, 0.0]),
        Vec3::zero(),
        Material::new(
            Color::new(0.8, 0.6, 0.2),
            Color::new(0.8, 0.6, 0.2),
            0.0,
            1.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([10.0, 20.0, 15.0]),
        12.0,
        Material::new(
            Color::new(1.0, 1.0, 1.0),
            Color::new(1.0, 1.0, 1.0),
            2.0,
            0.0
    ))));
   
    let image = scene.render(&camera, MAX_BOUNCE_COUNT, RAY_COUNT);
    image.save_png("./examples/teapots/image.png");
}