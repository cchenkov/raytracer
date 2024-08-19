use std::env;
use std::time;
use std::fs::File;
use std::io::{BufWriter, Write};

use raytracer::math::{div_up, clamp};
use raytracer::vec3::Vec3;
use raytracer::ray::Ray;
use raytracer::hit::{HitRecord, Hit};
use raytracer::sphere::Sphere;
use raytracer::box3::Box3;
use raytracer::camera::Camera;
use raytracer::progressbar::ProgressBar;
use raytracer::transform::{translation_matrix, scaling_matrix, x_rotation_matrix, y_rotation_matrix};
use raytracer::brdf::{d, g, f};

use Vec3 as Point3;
use Vec3 as Color;

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

fn shade(hit_record: &HitRecord, ray: &Ray, light: &Vec3) -> Color {
    let hit_point = hit_record.point;
    let normal = hit_record.normal.normalized();
    let view_dir = -ray.direction().normalized();
    let light_dir = (*light - hit_point).normalized();
    let halfway = (view_dir + light_dir).normalized();

    let alpha = 0.5;

    let f0 = Vec3::new(0.17, 0.17, 0.17, false);
    let ks = f(f0, view_dir, halfway);
    let kd = Vec3::new(1.0, 1.0, 1.0, false) - ks;

    let lambert = hit_record.color / std::f64::consts::PI;

    let d_val = d(alpha, normal, halfway);
    let g_val = g(alpha, normal, view_dir, light_dir);
    let f_val = f(f0, view_dir, halfway);

    let numerator = d_val * g_val * f_val;
    let denominator = (4.0 * normal.dot(view_dir).max(0.0) * normal.dot(light_dir).max(0.0)).max(0.000001);

    let cook_torrance = numerator / denominator;
    let brdf = kd * lambert + cook_torrance;
    let light_color = Vec3::new(4.0, 4.0, 4.0, false);

    brdf * light_color * normal.dot(light_dir).max(0.0)
}

fn ray_color(world: &Vec<Box<dyn Hit>>, ray: &Ray, light: &Vec3, t_min: f64, t_max: f64) -> Color {
    match trace_ray(world, ray, t_min, t_max) {
        Some(hit_record) => shade(&hit_record, ray, light),
        None => Color::new(0.5, 0.5, 0.5, false)
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    // image
    let path = &args[1];
    let aspect_ratio = 1.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 4;

    // output file
    let file = File::create(path).expect("Unable to open file");
    let mut file_writer = BufWriter::new(file);

    // camera
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 10.0, true), 
        Point3::new(0.0, 0.0, -1.0, true), 
        Vec3::new(0.0, 1.0, 0.0, false), 
        90.0, 
        aspect_ratio,
    );

    // ppm file format header
    write!(file_writer, "P3\n{} {}\n255\n", image_width, image_height).expect("Unable to write file");

    // objects
    let color_multiplier = 1.0 / 255.0;
    let green_color = Color::new(34.0, 139.0, 34.0, false) * color_multiplier;
    let red_color = Color::new(196.0, 30.0, 58.0, false) * color_multiplier;
    let _background = Color::new(127.0, 127.0, 127.0, false) * color_multiplier;
    let _translation = translation_matrix(&Vec3::new(-1.0, 0.0, 0.0, false));
    let scaling_x1 = scaling_matrix(1.0, 1.0, 1.0);
    let scaling_x2 = scaling_matrix(2.0, 2.0, 2.0);
    let rotation_x45 = x_rotation_matrix(30.0);
    let rotation_y45 = y_rotation_matrix(45.0);
    let _sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0, true), 1.0, green_color, Some(scaling_x1));
    let cube = Box3::new(Point3::new(-1.0, -1.0, -1.0, true), Point3::new(1.0, 1.0, 1.0, true), red_color, Some(rotation_x45 * rotation_y45 * scaling_x2));

    // world
    let mut world: Vec<Box<dyn Hit>> = Vec::new();
    // world.push(Box::new(sphere));
    world.push(Box::new(cube));

    // progress bar
    let length: usize = 50;
    let total: usize = (image_width * image_height).try_into().unwrap();
    let mut stdout = std::io::stdout();
    let mut progress_bar = ProgressBar::new(total, length, &mut stdout);

    // render
    let t_min: f64 = 0.001;
    let t_max: f64 = f64::INFINITY;
    // let light = Vec3::new(0.25, 0.5, 0.75, false).normalized();
    let light = Vec3::new(2.0, 4.0, 6.0, false);
    let timer = time::Instant::now();

    println!("\nRendering started...\n");

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0, false);

            for dx in (-samples_per_pixel / 2)..div_up(samples_per_pixel, 2) {
                for dy in (-samples_per_pixel / 2)..div_up(samples_per_pixel, 2) {
                    let u = (f64::from(x) + f64::from(dx) / f64::from(samples_per_pixel)) / f64::from(image_width - 1);
                    let v = (f64::from(y) + f64::from(dy) / f64::from(samples_per_pixel)) / f64::from(image_height - 1);
                    let ray = camera.get_ray(u, v);
                    pixel_color = pixel_color + ray_color(&world, &ray, &light, t_min, t_max);
                }
            }

            let scale = 1.0 / f64::from(samples_per_pixel * samples_per_pixel);

            let r = pixel_color.x() * scale;
            let g = pixel_color.y() * scale;
            let b = pixel_color.z() * scale;

            let ir = (255.0 * clamp(r, 0.0, 1.0)) as i32;
            let ig = (255.0 * clamp(g, 0.0, 1.0)) as i32;
            let ib = (255.0 * clamp(b, 0.0, 1.0)) as i32;

            writeln!(file_writer, "{} {} {}", ir, ig, ib).expect("Unable to write file");

            progress_bar.increment(1);
        }
    }

    println!("\n\nRendering finished...\nElapsed time: {}ms\n", timer.elapsed().as_millis());
}
