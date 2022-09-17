use crate::vec3::Vec3;
use crate::ray::Ray;

use Vec3 as Color;

pub trait Hit {
    fn hit(&self, ray: &Ray) -> Option<Color>;
}
