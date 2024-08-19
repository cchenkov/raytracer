use std::ops::Mul;
use crate::math::multiply;
use crate::vec3::Vec3;

pub trait Transform {
    type Output;

    fn transform(&self, matrix: &[[f64; 4]; 4]) -> Self::Output;
}

pub struct TransformMatrix {
    pub mat: [[f64; 4]; 4],
    pub inv: [[f64; 4]; 4]
}

impl Mul for TransformMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        TransformMatrix { 
            mat: multiply(&self.mat, &rhs.mat),
            inv: multiply(&rhs.inv, &self.inv), 
        }
    }
}

pub fn translation_matrix(delta: &Vec3) -> TransformMatrix {
    TransformMatrix {
        mat: [[1.0, 0.0, 0.0, delta.x()],
              [0.0, 1.0, 0.0, delta.y()],
              [0.0, 0.0, 1.0, delta.z()],
              [0.0, 0.0, 0.0,       1.0]],
        inv: [[1.0, 0.0, 0.0, -delta.x()],
              [0.0, 1.0, 0.0, -delta.y()],
              [0.0, 0.0, 1.0, -delta.z()],
              [0.0, 0.0, 0.0,        1.0]]
    }
}

pub fn scaling_matrix(x: f64, y: f64, z: f64) -> TransformMatrix {
    TransformMatrix {
        mat: [[  x, 0.0, 0.0, 0.0],
              [0.0,   y, 0.0, 0.0],
              [0.0, 0.0,   z, 0.0],
              [0.0, 0.0, 0.0, 1.0]],
        inv: [[1.0 / x,     0.0,     0.0, 0.0],
              [    0.0, 1.0 / y,     0.0, 0.0],
              [    0.0,     0.0, 1.0 / z, 0.0],
              [    0.0,     0.0,     0.0, 1.0]]
    }
}

pub fn x_rotation_matrix(theta: f64) -> TransformMatrix {
    let theta_rad = theta.to_radians();
    let sin_theta = theta_rad.sin();
    let cos_theta = theta_rad.cos();

    TransformMatrix {
        mat: [[1.0,       0.0,       0.0,  0.0],
              [0.0, cos_theta, -sin_theta, 0.0],
              [0.0, sin_theta,  cos_theta, 0.0],
              [0.0,       0.0,        0.0, 1.0]],
        inv: [[1.0,        0.0,       0.0, 0.0],
              [0.0,  cos_theta, sin_theta, 0.0],
              [0.0, -sin_theta, cos_theta, 0.0],
              [0.0,        0.0,       0.0, 1.0]]
    }
}

pub fn y_rotation_matrix(theta: f64) -> TransformMatrix {
    let theta_rad = theta.to_radians();
    let sin_theta = theta_rad.sin();
    let cos_theta = theta_rad.cos();

    TransformMatrix {
        mat: [[ cos_theta, 0.0, sin_theta,  0.0],
              [       0.0, 1.0,       0.0,  0.0],
              [-sin_theta, 0.0, cos_theta,  0.0],
              [       0.0, 0.0,       0.0,  1.0]],
        inv: [[cos_theta, 0.0, -sin_theta,  0.0],
              [      0.0, 1.0,        0.0,  0.0],
              [sin_theta, 0.0,  cos_theta,  0.0],
              [      0.0, 0.0,        0.0,  1.0]]
    }
}

pub fn z_rotation_matrix(theta: f64) -> TransformMatrix {
    let theta_rad = theta.to_radians();
    let sin_theta = theta_rad.sin();
    let cos_theta = theta_rad.cos();

    TransformMatrix {
        mat: [[cos_theta, -sin_theta, 0.0, 0.0],
              [sin_theta,  cos_theta, 0.0, 0.0],
              [      0.0,        0.0, 1.0, 0.0],
              [      0.0,        0.0, 0.0, 1.0]],
        inv: [[ cos_theta, sin_theta, 0.0, 0.0],
              [-sin_theta, cos_theta, 0.0, 0.0],
              [       0.0,       0.0, 1.0, 0.0],
              [       0.0,       0.0, 0.0, 1.0]]
    }
}
