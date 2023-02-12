use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};

use Vec3 as Point3;
use Vec3 as Color;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub color: Color
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, color: Color) -> Sphere {
        Sphere {
            center,
            radius,
            color
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;

        if t < t_min || t > t_max {
            t = (-half_b + sqrtd) / a;

            if t < t_min || t > t_max {
                return None;
            }
        }

        let hit_point = ray.at(t);
        let normal = (hit_point - self.center) / self.radius;
        let front_face = ray.direction().dot(normal) < 0.0;

        Some(HitRecord {
            tmin: t,
            point: hit_point,
            normal: if front_face { normal } else { -normal },
            front_face,
            color: self.color
        })
    }
}

