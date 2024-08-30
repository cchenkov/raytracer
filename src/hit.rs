use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::transform::TransformMatrix;

use Vec3 as Point3;

pub struct HitRecord {
    pub t_min: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn transform_matrix(&self) -> Option<&TransformMatrix>;
}
