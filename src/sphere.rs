use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};
use crate::transform::{Transform, TransformMatrix};
use crate::math::transpose;

use Vec3 as Point3;
use Vec3 as Color;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub color: Color,
    pub transform_matrix: Option<TransformMatrix>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, color: Color, transform_matrix: Option<TransformMatrix>) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            transform_matrix
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_ray = *ray;

        if self.transform_matrix.is_some() {
            t_ray = t_ray.transform(&self.transform_matrix.as_ref().unwrap().inv);
        }

        let oc = t_ray.origin() - self.center;
        let a = t_ray.direction().length_squared();
        let half_b = t_ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // println!("{} {} {} {}", discriminant, half_b, a, c);

        if discriminant < 0.0 {
            // println!("discriminant is < 0");
            return None;
        }

        // println!("discriminant is > 0");

        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;

        if t < t_min || t > t_max {
            t = (-half_b + sqrtd) / a;

            if t < t_min || t > t_max {
                // println!("no root in range");
                return None;
            }
        }

        // println!("has roots");

        let mut hit_point = t_ray.at(t);
        let mut normal = (hit_point - self.center) / self.radius;

        if self.transform_matrix.is_some() {
            hit_point = hit_point.transform(&self.transform_matrix.as_ref().unwrap().mat);
            normal = normal.transform(&transpose(&self.transform_matrix.as_ref().unwrap().mat));
        }

        let front_face = t_ray.direction().dot(normal) < 0.0;

        Some(HitRecord {
            t_min: t,
            point: hit_point,
            normal: if front_face { normal } else { -normal },
            front_face,
            color: self.color
        })
    }
}
