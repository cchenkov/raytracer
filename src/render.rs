use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};
use crate::brdf::{brdf, perturb};
use crate::transform::Transform;
use crate::math::transpose;
use crate::light::Light;

use Vec3 as Color;

const LIGHT_SAMPLES_COUNT: i32 = 4;
const REFLECT_SAMPLES_COUNT: i32 = 4;

fn intersect_world(world: &Vec<Box<dyn Hit>>, ray: &Ray) -> Option<HitRecord> {
    let t_min: f64 = 0.001;
    let mut t_max: f64 = f64::INFINITY;
    let mut hit_record: Option<HitRecord> = None;

    for object in world {
        let mut t_ray = *ray;

        if let Some(transform_matrix) = object.transform_matrix() {
            t_ray = t_ray.transform(&transform_matrix.inv);
        }

        if let Some(mut record) = object.hit(&t_ray, t_min, t_max) {
            t_max = record.t_min;

            if let Some(transform_matrix) = object.transform_matrix() {
                record.point = record.point.transform(&transform_matrix.mat);
                record.normal = record.normal.transform(&transpose(&transform_matrix.inv)).normalized();
            }

            hit_record = Some(record);
        }
    }

    hit_record
}

pub fn trace_ray(world: &Vec<Box<dyn Hit>>, ray: &Ray, light: &Light, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.08, 0.18, 0.29, false);
    }

    if let Some(hit_record) = intersect_world(world, ray) {
        let view_dir = -ray.direction();

        let mut direct_illumination = Color::new(0.0, 0.0, 0.0, false);

        for _ in 0..LIGHT_SAMPLES_COUNT {
            let light_dir = light.sample() - hit_record.point;
            let brdf = brdf(hit_record.material, hit_record.normal, view_dir, light_dir);
            let shadow_ray = Ray::new(hit_record.point, light_dir);

            if let None = intersect_world(world, &shadow_ray) {
                direct_illumination = direct_illumination + brdf * light.color * hit_record.normal.dot(light_dir).max(0.0);
            }
        }

        direct_illumination = direct_illumination / (LIGHT_SAMPLES_COUNT as f64);

        let reflect_dir = ray.direction().reflect(hit_record.normal);
        let mut indirect_illumination = Color::new(0.0, 0.0, 0.0, false);

        for _ in 0..REFLECT_SAMPLES_COUNT {
            let direction = perturb(&reflect_dir, hit_record.material.roughness);
            let reflect_ray = Ray::new(hit_record.point, direction);
            indirect_illumination = indirect_illumination + trace_ray(world, &reflect_ray, light, depth - 1);
        }

        indirect_illumination = indirect_illumination / (REFLECT_SAMPLES_COUNT as f64);

        return direct_illumination + indirect_illumination * hit_record.material.metallic;
    }

    Color::new(0.08, 0.18, 0.29, false)
}

