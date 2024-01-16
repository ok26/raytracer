use raytracer::{
    camera::Camera,
    scene::Scene,
    util::vector::Vec3,
    image::color::Color,
    objects::{mesh::Mesh, material::Material, sphere::Sphere}
};

const IMAGE_WIDTH: usize = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const FOV: f64 = 60.0;
const FOCAL_LENGTH: f64 = 2.0;

const MAX_BOUNCE_COUNT: usize = 15;
const RAY_COUNT: usize = 500;

const SUN_STRENGTH: f64 = 0.4;

fn main() {
    let camera = Camera::new(
        Vec3::new([0.0, 25.0, 1.0]),
        Vec3::new([0.0, -10.0, 0.0]),
        IMAGE_WIDTH,
        FOV,
        FOCAL_LENGTH,
        ASPECT_RATIO,
    );
    
    let mut scene = Scene::new(camera.clone(), SUN_STRENGTH);
    let ref pi = std::f64::consts::PI;
    let square_data = include_str!("square");

    for d in 0..15 {
        let cnt = 2 * d;
        for b in 0..cnt {
            let p = -pi + 2.0 * pi * b as f64 / cnt as f64;
            if p == 0.0 || p == -pi { continue; }
            scene.add_object(Box::new(Mesh::from_vex(
                square_data, 
                Vec3::new([f64::cos(p) * (40 - cnt) as f64 / 1.5, -d as f64, f64::sin(p) * (40 - cnt) as f64 / 1.5]), 
                Vec3::new([0.0, p * -57.0, 0.0]), 
                (40 - cnt) as f64 / 6.0, 
                Material::new(
                    Color::new(f64::cos(p).abs(), b as f64 / cnt as f64, f64::sin(p).abs()),
                    Color::white(),
                    0.0,
                    0.0
                ))));
        }   
    }
    scene.add_object(Box::new(Sphere::new(
        Vec3::new([0.0, -30.0, 0.0]), 
        10.0,
        Material::new(
            Color::new(0.2, 0.2, 0.2),
            Color::white(),
            0.0,
            0.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([0.0, 35.0, 0.0]), 
        20.0,
        Material::new(
            Color::white(),
            Color::white(),
            3.2,
            0.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([-30.0, -5.0, 0.0]), 
        15.0,
        Material::new(
            Color::new(0.8, 0.6, 0.4),
            Color::white(),
            0.0,
            1.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([30.0, -5.0, 0.0]), 
        15.0,
        Material::new(
            Color::new(0.8, 0.6, 0.4),
            Color::white(),
            0.0,
            1.0
    ))));

    let image = scene.render(&camera, MAX_BOUNCE_COUNT, RAY_COUNT);
    image.save_png("./examples/chaos/image.png");
}