use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};
use crate::transform::{Transform, TransformMatrix};
use crate::math::transpose;

use Vec3 as Point3;
use Vec3 as Color;

pub struct Box3 {
    pub min_bound: Point3,
    pub max_bound: Point3,
    pub color: Color,
    pub transform_matrix: Option<TransformMatrix>
}

impl Box3 {
    pub fn new(min_bound: Point3, max_bound: Point3, color: Color, transform_matrix: Option<TransformMatrix>) -> Box3 {
        Box3 {
            min_bound,
            max_bound,
            color,
            transform_matrix
        }
    }

    pub fn normal_at(&self, point: Point3) -> Vec3 {
        let normal_x: f64 = if (point.x() - self.min_bound.x()).abs() < 0.000001
                            || (point.x() - self.max_bound.x()).abs() < 0.000001
                            { 1.0 } else { 0.0 };

        let normal_y: f64 = if (point.y() - self.min_bound.y()).abs() < 0.000001
                            || (point.y() - self.max_bound.y()).abs() < 0.000001
                            { 1.0 } else { 0.0 };

        let normal_z: f64 = if (point.z() - self.min_bound.z()).abs() < 0.000001
                            || (point.z() - self.max_bound.z()).abs() < 0.000001
                            { 1.0 } else { 0.0 };

        Vec3::new(normal_x, normal_y, normal_z, false).normalized()
    }
}

impl Hit for Box3 {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_ray = *ray;

        if self.transform_matrix.is_some() {
            t_ray = ray.transform(&self.transform_matrix.as_ref().unwrap().inv);
        }

        let mut tmin: f64 = t_min;
        let mut tmax: f64 = t_max;

        for i in 0..3 {
            let t1 = (self.min_bound[i] - t_ray.origin()[i]) * t_ray.direction_inv()[i];
            let t2 = (self.max_bound[i] - t_ray.origin()[i]) * t_ray.direction_inv()[i];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        if tmin > tmax {
            return None;
        }
        
        let mut hit_point = t_ray.at(tmin);
        let mut normal = self.normal_at(hit_point);
        let front_face = t_ray.direction().dot(normal) < 0.0;

        if self.transform_matrix.is_some() {
            hit_point = hit_point.transform(&self.transform_matrix.as_ref().unwrap().mat);
            normal = normal.transform(&transpose(&self.transform_matrix.as_ref().unwrap().inv));
        }

        Some(HitRecord {
            t_min: tmin,
            point: hit_point,
            normal: if front_face { normal } else { -normal },
            front_face,
            color: self.color
        })
    }
}
