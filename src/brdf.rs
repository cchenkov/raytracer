use crate::vec3::Vec3;
use crate::material::Material;

pub fn d(alpha: f64, n: Vec3, h: Vec3) -> f64 {
    let alpha2 = alpha.powf(2.0);
    let n_dot_h2 = n.dot(h).max(0.0).powf(2.0);

    alpha2 / (std::f64::consts::PI * (n_dot_h2 * (alpha2 - 1.0) + 1.0).powf(2.0)).max(0.000001)
}

pub fn g1(alpha: f64, n: Vec3, x: Vec3) -> f64 {
    let n_dot_x = n.dot(x).max(0.0);
    let k = alpha / 2.0;

    n_dot_x / (n_dot_x * (1.0 - k) + k).max(0.000001)
}

pub fn g(alpha: f64, n: Vec3, v: Vec3, l: Vec3) -> f64 {
    g1(alpha, n, v) * g1(alpha, n, l)
}

pub fn f(f0: Vec3, v: Vec3, h: Vec3) -> Vec3 {
    f0 + (Vec3::new(1.0, 1.0, 1.0, false) - f0) * (1.0 - v.dot(h).max(0.0)).powf(5.0)
}

pub fn brdf(material: Material, n: Vec3, v: Vec3, l: Vec3) -> Vec3 {
    let h = (v + l).normalized();
    let f0 = Vec3::new(0.3, 0.3, 0.3, false) * (1.0 - material.metallic) + material.albedo * material.metallic;
    let f = f(f0, v, h);
    let d = d(material.roughness, n, h);
    let g = g(material.roughness, n, v, l);

    let numerator = d * g * f;
    let denominator = (4.0 * n.dot(v).max(0.0) * n.dot(l).max(0.0)).max(0.000001);
    let specular = numerator / denominator;
    let diffuse = material.albedo / std::f64::consts::PI;

    let ks = f;
    let kd = (Vec3::new(1.0, 1.0, 1.0, false) - ks) * (1.0 - material.metallic);

    kd * diffuse + ks * specular
}