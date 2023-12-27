use raytracer::{
    camera::Camera,
    scene::Scene,
    util::vector::Vec3,
    image::color::Color,
    objects::{mesh::Mesh, material::Material, sphere::Sphere, plane::Plane}
};

const IMAGE_WIDTH: usize = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const FOV: f64 = 90.0;
const FOCAL_LENGTH: f64 = 2.0;

const MAX_BOUNCE_COUNT: usize = 10;
const RAY_COUNT: usize = 1000;

const SUN_STRENGTH: f64 = 0.6;

fn main() {
    let camera = Camera::new(
        Vec3::new([4.0, 0.0, 0.0]),
        Vec3::zero(),
        IMAGE_WIDTH,
        FOV,
        FOCAL_LENGTH,
        ASPECT_RATIO
    );
    
    let mut scene = Scene::new(camera.clone(), SUN_STRENGTH);

    let box_data = include_str!("box");

    scene.add_object(Box::new(Mesh::from_vex(
        box_data,
        Vec3::new([0.0, 1.0, 3.0]),
        Vec3::new([65.0, 32.0, 40.0]),
        1.5,
        Material::new(
            Color::new(0.7, 0.1, 0.7),
            Color::blank(),
            0.0,
            0.0
    ))));

    scene.add_object(Box::new(Mesh::from_vex(
        box_data,
        Vec3::new([-2.0, 1.5, -3.0]),
        Vec3::new([10.0, -32.0, 20.0]),
        1.0,
        Material::new(
            Color::new(0.5, 1.0, 0.7),
            Color::blank(),
            0.0,
            0.0
    ))));

    scene.add_object(Box::new(Mesh::from_vex(
        box_data,
        Vec3::new([0.0, -0.7, 0.0]),
        Vec3::new([40.0, 5.0, -25.0]),
        2.0,
        Material::new(
            Color::new(0.8, 0.4, 0.1),
            Color::blank(),
            0.0,
            0.4
    ))));

    scene.add_object(Box::new(Mesh::from_vex(
        box_data,
        Vec3::new([3.0, -0.4, -1.0]),
        Vec3::new([20.0, 5.0, -40.0]),
        0.5,
        Material::new(
            Color::new(0.3, 0.8, 0.8),
            Color::new(0.3, 0.8, 0.8),
            0.6,
            0.0
    ))));

    scene.add_object(Box::new(Sphere::new(
        Vec3::new([6.0, 20.0, 5.0]),
        12.0,
        Material::new(
            Color::new(1.0, 1.0, 1.0),
            Color::new(1.0, 1.0, 1.0),
            1.0,
            0.0
    ))));

    scene.add_object(Box::new(Plane::new(
        Vec3::new([0.15, -1.0, 0.0]),
        Vec3::new([0.0, -1.0, 0.0]),
        Material::new(
            Color::new(0.2, 0.2, 0.75),
            Color::blank(),
            0.0,
            0.0
    ))));

    let image = scene.render(&camera, MAX_BOUNCE_COUNT, RAY_COUNT);
    image.save_png("./examples/transformations/image.png");
}