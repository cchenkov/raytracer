use crate::vec3::Vec3;
use crate::ray::Ray;

use Vec3 as Point3;
use Vec3 as Color;

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

    pub fn hit(&self, ray: &Ray) -> Option<Color> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = ray.direction().dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t = (-half_b - discriminant.sqrt()) / a;

        let hit_point = ray.origin() + ray.direction() * t;
        let normal = hit_point.normalized();
        let light = Vec3::new(1.0, 1.0, -1.0).normalized();

        let color = Color::new(1.0, 0.0, 0.0) * normal.dot(light).max(0.0);

        return Some(color);
    }
}

