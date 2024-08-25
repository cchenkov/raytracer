use rand::Rng;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hit};
use crate::brdf::brdf;

use Vec3 as Color;

const LIGHT_RADIUS: f64 = 0.2;
const LIGHT_SAMPLES_COUNT: i32 = 4;
const REFLECT_SAMPLES_COUNT: i32 = 4;

fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let mut rng = rand::thread_rng();
    let phi = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
    let cos_theta = rng.gen::<f64>();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let random_dir = Vec3::new(
        sin_theta * phi.cos(),
        sin_theta * phi.sin(),
        cos_theta,
        false
    );

    if random_dir.dot(*normal) < 0.0 {
        -random_dir
    } else {
        random_dir
    }
}

fn random_in_cone(reflect_dir: &Vec3, roughness: f64) -> Vec3 {
    let mut rng = rand::thread_rng();

    let phi = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
    let cos_theta = (1.0 - rng.gen::<f64>()).powf(1.0 / (roughness + 1.0));
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let random_dir = Vec3::new(
        sin_theta * phi.cos(),
        sin_theta * phi.sin(),
        cos_theta,
        false
    );

    let up = if reflect_dir.z().abs() < 0.999 {
        Vec3::new(0.0, 0.0, 1.0, false)
    } else {
        Vec3::new(1.0, 0.0, 0.0, false)
    };

    let tangent = up.cross(*reflect_dir).normalized();
    let bitangent = reflect_dir.cross(tangent);

    let perturbed_dir = Vec3::new(
        tangent.x() * random_dir.x() + bitangent.x() * random_dir.y() + reflect_dir.x() * random_dir.z(),
        tangent.y() * random_dir.x() + bitangent.y() * random_dir.y() + reflect_dir.y() * random_dir.z(),
        tangent.z() * random_dir.x() + bitangent.z() * random_dir.y() + reflect_dir.z() * random_dir.z(),
        false
    );

    perturbed_dir.normalized()
}

fn perturb_reflection(reflect_dir: &Vec3, roughness: f64) -> Vec3 {
    let mut rng = rand::thread_rng();

    let perturbation = Vec3::new(
        rng.gen::<f64>() * 2.0 - 1.0,
        rng.gen::<f64>() * 2.0 - 1.0,
        rng.gen::<f64>() * 2.0 - 1.0,
        false
    );

    (*reflect_dir + perturbation * roughness).normalized()
}

fn sample_light(light_pos: &Vec3, light_radius: f64, samples_count: i32) -> Vec<Vec3> {
    let mut samples = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..samples_count {
        let theta = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
        let r = light_radius * (rng.gen::<f64>()).sqrt();
        let x = r * theta.cos();
        let y = r * theta.sin();

        samples.push(*light_pos + Vec3::new(x, y, 0.0, false));
    }

    samples
}

fn intersect_world(world: &Vec<Box<dyn Hit>>, ray: &Ray) -> Option<HitRecord> {
    let t_min: f64 = 0.001;
    let mut t_max: f64 = f64::INFINITY;
    let mut hit_record: Option<HitRecord> = None;

    for object in world {
        if let Some(record) = object.hit(ray, t_min, t_max) {
            t_max = record.t_min;
            hit_record = Some(record);
        }
    }

    hit_record
}

pub fn trace_ray(world: &Vec<Box<dyn Hit>>, ray: &Ray, light_pos: &Vec3, light_color: &Vec3, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.08, 0.18, 0.29, false);
    }

    if let Some(hit_record) = intersect_world(world, ray) {
        let view_dir = -ray.direction();
        // let light_dir = *light_pos - hit_record.point;

        let ligth_samples = sample_light(light_pos, LIGHT_RADIUS, LIGHT_SAMPLES_COUNT);
        let mut direct_illumination = Color::new(0.0, 0.0, 0.0, false);

        for sample in ligth_samples {
            let light_dir = sample - hit_record.point;
            let brdf = brdf(hit_record.material, hit_record.normal, view_dir, light_dir);
            let shadow_ray = Ray::new(hit_record.point, light_dir);

            if let None = intersect_world(world, &shadow_ray) {
                direct_illumination = direct_illumination + brdf * *light_color * hit_record.normal.dot(light_dir).max(0.0);
            }
        }

        direct_illumination = direct_illumination / (LIGHT_SAMPLES_COUNT as f64);

        // let brdf = brdf(hit_record.material, hit_record.normal, view_dir, light_dir);
        // let direct_illumination = brdf * *light_color * hit_record.normal.dot(light_dir).max(0.0);

        let reflect_dir = ray.direction() - 2.0 * hit_record.normal * ray.direction().dot(hit_record.normal);
        let reflect_ray = Ray::new(hit_record.point, reflect_dir);
        let reflect_color = trace_ray(world, &reflect_ray, light_pos, light_color, depth - 1);

        // let reflect_dir = ray.direction() - 2.0 * hit_record.normal * ray.direction().dot(hit_record.normal);
        // let mut reflect_color = Color::new(0.0, 0.0, 0.0, false);

        // for _ in 0..REFLECT_SAMPLES_COUNT {
        //     // let direction = random_in_cone(&reflect_dir, hit_record.material.roughness);
        //     // let reflect_ray = Ray::new(hit_record.point, direction);
        //     // let bias = random_in_hemisphere(&hit_record.normal) * hit_record.material.roughness;
        //     // let reflect_ray = Ray::new(hit_record.point, (reflect_dir + bias).normalized());
        //     let direction = perturb_reflection(&reflect_dir, hit_record.material.roughness);
        //     let reflect_ray = Ray::new(hit_record.point, direction);
        //     reflect_color = reflect_color + trace_ray(world, &reflect_ray, light_pos, light_color, depth - 1);
        // }

        // reflect_color = reflect_color / (REFLECT_SAMPLES_COUNT as f64);

        return direct_illumination + reflect_color * hit_record.material.metallic;
    }

    Color::new(0.08, 0.18, 0.29, false)
}

