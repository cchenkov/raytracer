use std::env;
use std::time;
use std::fs::File;
use std::io::{BufWriter, Write};

use raytracer::vec3::Vec3;
use raytracer::hit::Hit;
use raytracer::sphere::Sphere;
use raytracer::box3::Box3;
use raytracer::camera::Camera;

use Vec3 as Point3;
use Vec3 as Color;

fn main() {
    let args : Vec<String> = env::args().collect();

    // image
    let path = &args[1];
    let aspect_ratio = 1.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 8;

    // output file
    let file = File::create(path).expect("Unable to open file");
    let mut file_writer = BufWriter::new(file);

    // camera
    let camera = Camera::new(
        Point3::new(3.0, 3.0, 6.0), 
        Point3::new(0.0, 0.0, -1.0), 
        Vec3::new(0.0, 1.0, 0.0), 
        90.0, 
        aspect_ratio,
    );

    // ppm file format header
    write!(file_writer, "P3\n{} {}\n255\n", image_width, image_height).expect("Unable to write file");

    // objects
    let red_color = Color::new(196.0 / 255.0, 30.0 / 255.0, 58.0 / 255.0);
    let background = Color::new(0.5, 0.5, 0.5);
    let _sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 1.0, red_color);
    let cube = Box3::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0), red_color);

    // render
    let timer = time::Instant::now();

    println!("Rendering started...");

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for dx in (-samples_per_pixel / 2)..(samples_per_pixel / 2) {
                for dy in (-samples_per_pixel / 2)..(samples_per_pixel / 2) {
                    let u = (f64::from(x) + f64::from(dx) / f64::from(samples_per_pixel)) / f64::from(image_width - 1);
                    let v = (f64::from(y) + f64::from(dy) / f64::from(samples_per_pixel)) / f64::from(image_height - 1);

                    let ray = camera.get_ray(u, v);

                    pixel_color = pixel_color + match cube.hit(&ray) {
                        None => background,
                        Some(color) => color
                    };
                }
            }

            let scale = 1.0 / f64::from(samples_per_pixel * samples_per_pixel);

            let r = pixel_color.x() * scale;
            let g = pixel_color.y() * scale;
            let b = pixel_color.z() * scale;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(file_writer, "{} {} {}", ir, ig, ib).expect("Unable to write file");
        }
    }

    println!("Rendering finished. Elapsed time: {}ms", timer.elapsed().as_millis());
}
