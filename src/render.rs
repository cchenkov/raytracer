use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};
use crate::brdf::{d, g, f};

use Vec3 as Color;

pub fn ray_color(world: &Vec<Box<dyn Hit>>, ray: &Ray, light: &Vec3, t_min: f64, t_max: f64) -> Color {
    match trace_ray(world, ray, t_min, t_max) {
        Some(hit_record) => shade(world, &hit_record, ray, light),
        None => Color::new(0.52, 0.80, 0.92, false)
    }
}

fn trace_ray(world: &Vec<Box<dyn Hit>>, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest: f64 = t_max;
    let mut hit_record: Option<HitRecord> = None;

    for object in world {
        if let Some(record) = object.hit(ray, t_min, closest) {
            closest = record.t_min;
            hit_record = Some(record);
        }
    }

    hit_record
}

fn cast_shadow(world: &Vec<Box<dyn Hit>>, hit_point: &Vec3, light: &Vec3) -> f64 {
    let num_shadow_rays = 1;
    let light_radius = 1.0;
    let mut shadow_factor = 0.0;

    for _ in 0..num_shadow_rays {
        let offset_x = (rand::random::<f64>() - 0.5) * light_radius;
        let offset_y = (rand::random::<f64>() - 0.5) * light_radius;
        let offset_z = (rand::random::<f64>() - 0.5) * light_radius;
        
        let offset_light = *light + Vec3::new(offset_x, offset_y, offset_z, false);
        let shadow_ray_dir = (offset_light - *hit_point).normalized();
        let shadow_ray = Ray::new(*hit_point, shadow_ray_dir);

        let mut in_shadow = false;

        for object in world {
            if let Some(_) = object.hit(&shadow_ray, 0.001, f64::INFINITY) {
                in_shadow = true;
                break;
            }
        }

        if !in_shadow {
            shadow_factor += 1.0;
        }
    }

    shadow_factor / num_shadow_rays as f64
}

fn shade(world: &Vec<Box<dyn Hit>>, hit_record: &HitRecord, ray: &Ray, light: &Vec3) -> Color {
    let hit_point = hit_record.point;
    let normal = hit_record.normal.normalized();
    let view_dir = -ray.direction().normalized();
    let light_dir = (*light - hit_point).normalized();
    let halfway = (view_dir + light_dir).normalized();

    let shadow_factor = cast_shadow(world, &hit_point, light);

    let alpha = 0.25;

    let f0 = Vec3::new(0.08, 0.08, 0.08, false);
    let ks = f(f0, view_dir, halfway);
    let kd = Vec3::new(1.0, 1.0, 1.0, false) - ks;

    let lambert = hit_record.color / std::f64::consts::PI;

    let d_val = d(alpha, normal, halfway);
    let g_val = g(alpha, normal, view_dir, light_dir);
    let f_val = f(f0, view_dir, halfway);

    let numerator = d_val * g_val * f_val;
    let denominator = (4.0 * normal.dot(view_dir).max(0.0) * normal.dot(light_dir).max(0.0)).max(0.000001);

    let cook_torrance = numerator / denominator;
    let brdf = kd * lambert + ks * cook_torrance;
    let light_color = Vec3::new(4.0, 4.0, 4.0, false);

    brdf * light_color * normal.dot(light_dir).max(0.0) * shadow_factor
}

