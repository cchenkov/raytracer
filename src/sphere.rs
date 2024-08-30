use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};
use crate::material::Material;
use crate::transform::TransformMatrix;

use Vec3 as Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
    pub transform_matrix: Option<TransformMatrix>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material, transform_matrix: Option<TransformMatrix>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            transform_matrix
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
            t_min: t,
            point: hit_point,
            normal: if front_face { normal } else { -normal },
            front_face,
            material: self.material
        })
    }

    fn transform_matrix(&self) -> Option<&TransformMatrix> {
        self.transform_matrix.as_ref()
    }
}
