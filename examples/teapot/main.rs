use raytracer::{
    camera::Camera,
    scene::Scene,
    util::vector::Vec3,
    image::color::Color,
    objects::{mesh::Mesh, material::Material}
};

const IMAGE_WIDTH: usize = 320;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const FOV: f64 = 90.0;
const FOCAL_LENGTH: f64 = 2.0;

const MAX_BOUNCE_COUNT: usize = 5;
const RAY_COUNT: usize = 1;

const SUN_STRENGTH: f64 = 1.0;

fn main() {
    let camera = Camera::new(
        Vec3::new([10.0, 4.0, 0.0]),
        Vec3::zero(),
        IMAGE_WIDTH,
        FOV,
        FOCAL_LENGTH,
        ASPECT_RATIO,
    );
    
    let mut scene = Scene::new(camera.clone(), SUN_STRENGTH);

    let teapot = Mesh::from_obj(
        include_str!("teapot.obj"),
        Vec3::new([0.0, 0.0, 0.0]),
        Material::new(
            Color::new(0.8, 0.8, 0.8),
            Color::blank(),
            0.0,
            0.0
    ));
   
    scene.add_object(&teapot);
    let image = scene.render(&camera, MAX_BOUNCE_COUNT, RAY_COUNT);
    image.save_png("./examples/teapot/image.png");
}