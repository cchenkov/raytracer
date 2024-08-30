use rand::Rng;

use crate::vec3::Vec3;

use Vec3 as Point3;
use Vec3 as Color;

pub struct Light {
    pub color: Color,
    pub position: Point3,
    pub radius: f64
}

impl Light {
    pub fn new(color: Color, position: Point3, radius: f64) -> Light {
        Light {
            color,
            position,
            radius
        }
    }

    pub fn sample(&self) -> Vec3 {
        let mut rng = rand::thread_rng();

        let theta = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
        let r = self.radius * (rng.gen::<f64>()).sqrt();
        let x = r * theta.cos();
        let y = r * theta.sin();

        self.position + Vec3::new(x, y, 0.0, false)
    }
}