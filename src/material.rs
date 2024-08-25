use crate::vec3::Vec3;

use Vec3 as Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub albedo: Color,
    pub roughness: f64,
    pub metallic: f64
}

impl Material {
    pub fn new(albedo: Color, roughness: f64, metallic: f64) -> Material {
        Material { albedo, roughness, metallic }
    }
}
