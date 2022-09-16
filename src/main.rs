use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use raytracer::vec3::Vec3;
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

    let background = Color::new(0.0, 0.0, 0.0);
    let _sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Color::new(1.0, 0.0, 0.0));
    let cube = Box3::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0), Color::new(1.0, 0.0, 0.0));

    // render
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let u = f64::from(j) / f64::from(image_width - 1);
            let v = f64::from(i) / f64::from(image_height - 1);

            let ray = camera.get_ray(u, v);

            let color = match cube.hit(&ray) {
                None => background,
                Some(color) => color
            };

            let ir = (255.999 * color.x()) as i32;
            let ig = (255.999 * color.y()) as i32;
            let ib = (255.999 * color.z()) as i32;

            writeln!(file_writer, "{} {} {}", ir, ig, ib).expect("Unable to write file");
        }
    }
}
