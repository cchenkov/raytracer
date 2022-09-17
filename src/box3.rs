use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;

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
}

impl Hit for Box3 {
    fn hit(&self, ray: &Ray) -> Option<Color> {
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

        let normal_x: f64 = if (hit_point.x() - self.min_bound.x()).abs() < 0.00001 
                            || (hit_point.x() - self.max_bound.x()).abs() < 0.00001 { 1.0 } else { 0.0 };
        let normal_y: f64 = if (hit_point.y() - self.min_bound.y()).abs() < 0.00001 
                            || (hit_point.y() - self.max_bound.y()).abs() < 0.00001 { 1.0 } else { 0.0 };
        let normal_z: f64 = if (hit_point.z() - self.min_bound.z()).abs() < 0.00001 
                            || (hit_point.z() - self.max_bound.z()).abs() < 0.00001 { 1.0 } else { 0.0 };

        let mut normal = Vec3::new(normal_x, normal_y, normal_z).normalized();
        let front_face = ray.direction().dot(normal) < 0.0;
        normal = if front_face { normal } else { -normal };

        let light = Vec3::new(0.25, 0.5, 0.75).normalized();

        Some(self.color * normal.dot(light).max(0.0))
    }
}

