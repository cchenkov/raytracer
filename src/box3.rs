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
        let mut normal: Vec3;

        if hit_point.x() == self.min_bound.x() || hit_point.x() == self.max_bound.x() {
            normal = Vec3::new(1.0, 0.0, 0.0);
        }
        else if hit_point.y() == self.min_bound.y() || hit_point.y() == self.max_bound.y() {
            normal = Vec3::new(0.0, 1.0, 0.0);
        }
        else {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }

        let light = Vec3::new(1.0, 1.0, 1.0).normalized();
        let front_face = ray.direction().dot(normal) < 0.0;

        normal = if front_face { normal } else { -normal };

        Some(self.color * normal.dot(light).max(0.0))
    }
}
