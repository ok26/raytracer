use crate::image::color::Color;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color::blank(); width * height]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Color) {
        self.pixels[y * self.width + x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn save_png(&self, name: &str) {
        use image::{Rgb, RgbImage};

        let mut img = RgbImage::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = self.get(x as usize, y as usize);
            *pixel = Rgb([
                (color.r * 255.0).round() as u8,
                (color.g * 255.0).round() as u8,
                (color.b * 255.0).round() as u8,
            ]);
        }
        img.save(name).unwrap();
    }
}