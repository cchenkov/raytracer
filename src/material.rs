use crate::vec3::Vec3;

use Vec3 as Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub reflectivity: f64
}

impl Material {
    pub fn new(color: Color, reflectivity: f64) -> Material {
        Material { color, reflectivity }
    }
}
