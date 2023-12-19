use crate::util::vector::Vec3;

#[derive(Clone)]
pub struct Camera {
    position: Vec3,
    delta_u: Vec3,
    delta_v: Vec3,
    viewport_upper_left: Vec3,
    image_height: usize,
    image_width: usize
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, image_width: usize, fov: f64, focal_length: f64, aspect_ratio: f64) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let theta = f64::to_radians(fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (position - look_at).normalized();
        let u = Vec3::new([0.0, 1.0, 0.0]).cross(w).normalized();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * (-viewport_height);

        let delta_u = viewport_u * (1.0 / image_width as f64);
        let delta_v = viewport_v * (1.0 / image_height as f64);

        let viewport_upper_left = position - w * focal_length - viewport_u * 0.5 - viewport_v * 0.5;

        Camera {
            position,
            delta_u,
            delta_v,
            viewport_upper_left,
            image_height,
            image_width
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn viewport_upper_left(&self) -> Vec3 {
        self.viewport_upper_left
    }

    pub fn delta_u(&self) -> Vec3 {
        self.delta_u
    }

    pub fn delta_v(&self) -> Vec3 {
        self.delta_v
    }

    pub fn image_height(&self) -> usize {
        self.image_height
    }

    pub fn image_width(&self) -> usize {
        self.image_width
    }
}