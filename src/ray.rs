use crate::vec3::Vec3;
use crate::transform::Transform;

use Vec3 as Point3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    direction_inv: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction, direction_inv: 1.0 / direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn direction_inv(&self) -> Vec3 {
        self.direction_inv
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

impl Transform for Ray {
    type Output = Self;

    fn transform(&self, matrix: &[[f64; 4]; 4]) -> Self::Output {
        let new_origin = self.origin.transform(matrix);
        let new_direction = self.direction.transform(matrix);
        let new_direction_inv = 1.0 / new_direction;

        Self::Output {
            origin: new_origin,
            direction: new_direction,
            direction_inv: new_direction_inv
        }
    }
}
