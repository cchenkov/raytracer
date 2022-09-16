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
        let mut tmin: f64 = 0.001;
        let mut tmax: f64 = f64::INFINITY;

        for i in 0..3 {
            let t1 = (self.min_bound[i] - ray.origin()[i]) * ray.direction_inv()[i];
            let t2 = (self.max_bound[i] - ray.origin()[i]) * ray.direction_inv()[i];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        if tmin > tmax {
            return None;
        }

        let hit_point = ray.at(tmin);

        let normal_x: f64 = if (hit_point.x() - self.min_bound.x()).abs() < 1.0 
                            || (hit_point.x() - self.max_bound.x()).abs() < 1.0 { hit_point.x() } else { 0.0 };
        let normal_y: f64 = if (hit_point.y() - self.min_bound.y()).abs() < 1.0 
                            || (hit_point.y() - self.max_bound.y()).abs() < 1.0 { hit_point.y() } else { 0.0 };
        let normal_z: f64 = if (hit_point.z() - self.min_bound.z()).abs() < 1.0 
                            || (hit_point.z() - self.max_bound.z()).abs() < 1.0 { hit_point.z() } else { 0.0 };

        let normal = Vec3::new(normal_x, normal_y, normal_z).normalized();
        let light = Vec3::new(0.0, 0.25, 1.0).normalized();

        Some(self.color * normal.dot(light).max(0.0))
    }
}
