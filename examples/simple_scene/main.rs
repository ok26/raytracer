use raytracer::{
    camera::Camera,
    scene::Scene,
    util::vector::Vec3,
    image::color::Color,
    objects::{plane::Plane, sphere:: Sphere, material::Material}
};

const IMAGE_WIDTH: usize = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const FOV: f64 = 90.0;
const FOCAL_LENGTH: f64 = 2.0;

const MAX_BOUNCE_COUNT: usize = 10;
const RAY_COUNT: usize = 100;

const SUN_STRENGTH: f64 = 0.6;

fn main() {
    let camera = Camera::new(
        Vec3::new([4.0, 10.0, -3.0]),
        Vec3::new([-20.0, -5.0, 0.0]),
        IMAGE_WIDTH,
        FOV,
        FOCAL_LENGTH,
        ASPECT_RATIO
    );

    let mut scene = Scene::new(camera.clone(), SUN_STRENGTH);

    let plane = Plane::new(
        Vec3::new([0.0, -1.0, 0.0]), //Flipped
        Vec3::new([0.0, -2.0, 0.0]),
        Material::new(
            Color::new(1.0, 0.0, 0.0),
            Color::new(1.0, 0.0, 0.0),
            0.0,
            0.0
    ));
    
    let sphere1 = Sphere::new(
        Vec3::new([-20.0, 0.0, 0.0]),
        10.0,
        Material::new(
            Color::new(0.9, 0.9, 0.9),
            Color::new(1.0, 1.0, 1.0),
            1.0,
            1.0
    ));

    let sphere2 = Sphere::new(
        Vec3::new([-10.5, -1.0, -5.5]),
        4.0,
        Material::new(
            Color::new(0.8, 0.8, 0.0),
            Color::new(0.8, 0.8, 0.0),
            0.8,
            0.0
    ));

    let sphere3 = Sphere::new(
        Vec3::new([-6.0, -2.0, 4.0]),
        2.0,
        Material::new(
            Color::new(0.6, 0.6, 1.0),
            Color::new(0.3, 0.3, 1.0),
            0.0,
            0.0
    ));

    let sphere4 = Sphere::new(
        Vec3::new([-4.0, 5.0, 3.0]),
        1.0,
        Material::new(
            Color::new(0.9, 0.9, 0.9),
            Color::new(1.0, 1.0, 1.0),
            0.0,
            1.0
    ));

    scene.add_object(&plane);
    scene.add_object(&sphere1);
    scene.add_object(&sphere2);
    scene.add_object(&sphere3);
    scene.add_object(&sphere4);
    let image = scene.render(&camera, MAX_BOUNCE_COUNT, RAY_COUNT);
    image.save_png("./examples/simple_scene/image.png");
}