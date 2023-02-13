use std::ops::{Add, Sub, Mul, Div, Neg, Index};
use crate::transform::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
    is_point: bool
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64, is_point: bool) -> Vec3 {
        Vec3 { x, y, z, is_point }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.length();

        Vec3::new(self.x / len, self.y / len, self.z / len, self.is_point)
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
            self.is_point
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            is_point: self.is_point
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            is_point: self.is_point
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            is_point: self.is_point
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            is_point: self.is_point
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            is_point: rhs.is_point
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            is_point: self.is_point
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            is_point: self.is_point
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
            is_point: rhs.is_point
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            is_point: self.is_point
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range")
        }
    }
}

impl Transform for Vec3 {
    type Output = Self;

    fn transform(&self, matrix: &[[f64; 4]; 4]) -> Self::Output {
        let mut xp = self.x * matrix[0][0] + self.y * matrix[0][1] + self.z * matrix[0][2];
        let mut yp = self.x * matrix[1][0] + self.y * matrix[1][1] + self.z * matrix[1][2];
        let mut zp = self.x * matrix[2][0] + self.y * matrix[2][1] + self.z * matrix[2][2];

        if self.is_point {
            xp = xp + matrix[0][3];
            yp = yp + matrix[1][3];
            zp = zp + matrix[2][3];

            let wp = self.x * matrix[3][0] + self.y * matrix[3][1] + self.z * matrix[3][2] + matrix[3][3];

            Self::Output { 
                x: xp / wp, 
                y: yp / wp, 
                z: zp / wp,
                is_point: self.is_point 
            }
        } else {
            Self::Output { 
                x: xp, 
                y: yp, 
                z: zp,
                is_point: self.is_point 
            } 
        }
    }
}
