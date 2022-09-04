use crate::vec3::Vec3;
use crate::ray::Ray;

use Vec3 as Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = ray.direction().dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;

        return true;
    }
}

