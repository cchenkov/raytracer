use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args : Vec<String> = env::args().collect();

    let path = &args[1];
    let width = 256;
    let height = 256;

    let file = File::create(path).expect("Unable to open file");

    let mut file_writer = BufWriter::new(file);

    // ppm file format header
    write!(file_writer, "P3\n{} {}\n255\n", width, height).expect("Unable to write file");

    for i in (0..height).rev() {
        for j in 0..width {
            let r = f64::from(j) / f64::from(width - 1);
            let g = f64::from(i) / f64::from(height - 1);
            let b = 0.25;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            write!(file_writer, "{} {} {}\n", ir, ig, ib).expect("Unable to write file");
        }
    }
}

