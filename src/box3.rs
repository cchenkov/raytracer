use crate::vec3::Vec3;
use crate::ray::Ray;

use Vec3 as Point3;
use Vec3 as Color;

pub struct Box3 {
    pub min_bound: Point3,
    pub max_bound: Point3,
    pub color: Color
}

impl Box3 {
    pub fn new(min_bound: Point3, max_bound: Point3, color: Color) -> Box3 {
        Box3 {
            min_bound,
            max_bound,
            color
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<Color> {
        let mut tmin: f64;
        let mut tmax: f64;
        let tymin: f64;
        let tymax: f64;
        let tzmin: f64;
        let tzmax: f64;

        if ray.direction().x() >= 0.0 {
            tmin = (self.min_bound.x() - ray.origin().x()) * ray.direction_inv().x();
            tmax = (self.max_bound.x() - ray.origin().x()) * ray.direction_inv().x();
        } else {
            tmin = (self.max_bound.x() - ray.origin().x()) * ray.direction_inv().x();
            tmax = (self.min_bound.x() - ray.origin().x()) * ray.direction_inv().x();
        }

        if ray.direction().y() >= 0.0 {
            tymin = (self.min_bound.y() - ray.origin().y()) * ray.direction_inv().y();
            tymax = (self.max_bound.y() - ray.origin().y()) * ray.direction_inv().y();
        } else {
            tymin = (self.max_bound.y() - ray.origin().y()) * ray.direction_inv().y();
            tymax = (self.min_bound.y() - ray.origin().y()) * ray.direction_inv().y();
        }

        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        if ray.direction().z() >= 0.0 {
            tzmin = (self.min_bound.z() - ray.origin().z()) * ray.direction_inv().z();
            tzmax = (self.max_bound.z() - ray.origin().z()) * ray.direction_inv().z();
        } else {
            tzmin = (self.max_bound.z() - ray.origin().z()) * ray.direction_inv().z();
            tzmax = (self.min_bound.z() - ray.origin().z()) * ray.direction_inv().z();
        }

        if tmin > tzmax || tzmin > tmax {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmin < f64::INFINITY && tmax > 0.000001 {
            return Some(self.color);
        }

        None
    }
}
