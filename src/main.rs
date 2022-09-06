use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use raytracer::vec3::Vec3;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::box3::Box3;

use Vec3 as Point3;
use Vec3 as Color;

fn main() {
    let args : Vec<String> = env::args().collect();

    // image
    let path = &args[1];
    let aspect_ratio = 1.0 / 1.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let file = File::create(path).expect("Unable to open file");

    let mut file_writer = BufWriter::new(file);

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 1.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // ppm file format header
    write!(file_writer, "P3\n{} {}\n255\n", image_width, image_height).expect("Unable to write file");

    let background = Color::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Color::new(1.0, 0.0, 0.0));
    let cube = Box3::new(Point3::new(-0.25, -0.25, -0.25), Point3::new(0.25, 0.25, 0.25), Color::new(1.0, 0.0, 0.0));

    // render
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let u = f64::from(j) / f64::from(image_width - 1);
            let v = f64::from(i) / f64::from(image_height - 1);

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let color = match cube.hit(&ray) {
                None => background,
                Some(color) => color
            };

            let ir = (255.999 * color.x()) as i32;
            let ig = (255.999 * color.y()) as i32;
            let ib = (255.999 * color.z()) as i32;

            write!(file_writer, "{} {} {}\n", ir, ig, ib).expect("Unable to write file");
        }
    }
}
