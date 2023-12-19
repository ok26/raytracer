use crate::image::color::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub emission_color: Color,
    pub emission_strength: f64,
    pub smoothness: f64
}

impl Material {
    pub fn new(color: Color, emission_color: Color, emission_strength: f64, smoothness: f64) -> Material {
        Material { 
            color,
            emission_color,
            emission_strength,
            smoothness
         }
    }

    pub fn blank() -> Material {
        Material {
            color: Color::white(),
            emission_color: Color::blank(),
            emission_strength: 0.0,
            smoothness: 0.0
        }
    }
}