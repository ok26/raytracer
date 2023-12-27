use raytracer::{
    camera::Camera,
    scene::Scene,
    util::vector::Vec3,
    image::color::Color,
    objects::{mesh::Mesh, material::Material, plane::Plane, sphere::Sphere}
};

const IMAGE_WIDTH: usize = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const FOV: f64 = 60.0;
const FOCAL_LENGTH: f64 = 2.0;

const MAX_BOUNCE_COUNT: usize = 5;
const RAY_COUNT: usize = 500;

const SUN_STRENGTH: f64 = 0.5;

fn main() {
    let camera = Camera::new(
        Vec3::new([0.0, 4.0, 7.0]),
        Vec3::new([0.0, 1.0, 0.0]),
        IMAGE_WIDTH,
        FOV,
        FOCAL_LENGTH,
        ASPECT_RATIO,
    );
    
    let mut scene = Scene::new(camera.clone(), SUN_STRENGTH);

    scene.add_object(Box::new(Mesh::from_obj(
        include_str!("teapot.obj"),
        Vec3::zero(),
        Vec3::zero(),
        1.0,
        Material::new(
            Color::new(0.8, 0.8, 0.8),
            Color::blank(),
            0.0,
            0.4
    ))));

    scene.add_object(Box::new(Plane::new(
        Vec3::new([0.0, -1.0, 0.0]),
        Vec3::zero(),
        Material::new(
            Color::new(0.0, 1.0, 0.0),
            Color::blank(),
            0.0,
            0.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([-5.0, 3.0, -2.0]),
        1.5,
        Material::new(
            Color::new(0.4, 0.4, 0.8),
            Color::new(1.0, 1.0, 1.0),
            0.8,
            0.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([10.0, 30.0, 6.0]),
        12.0,
        Material::new(
            Color::new(1.0, 1.0, 1.0),
            Color::new(1.0, 1.0, 1.0),
            1.0,
            0.0
    ))));
   
    let image = scene.render(&camera, MAX_BOUNCE_COUNT, RAY_COUNT);
    image.save_png("./examples/teapot/image2.png");
}