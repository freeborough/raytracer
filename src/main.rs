use std::fs::File;
use std::io::{BufWriter, Write};
use raytracer::{Vector3, unit_vector};

fn main() {
    println!("Raytracer v0.1");

    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let a = Vector3::new(2.0, 2.0, 2.0);
    println!("A: {}", a.length());
    let unit_a = unit_vector(&a);
    println!("Unit: {}", unit_a.length());


    // Render

    // Open output file for writing.
    let output_filename = "./output.ppm";
    let output_file = File::create(output_filename)
        .unwrap_or_else(|_| panic!("Could not open output file: {output_filename}"));
    let mut output_buffer = BufWriter::new(output_file);

    // Header
    let _bytes_written = output_buffer
        .write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    print!("Rendering");

    // Render loop.
    for j in 0..image_height {

        print!(".");

        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = 0.0;
            let b = j as f64 / (image_height as f64 - 1.0);

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            let _bytes_written = output_buffer
                .write(format!("{ir} {ig} {ib}\n").as_bytes())
                .unwrap();
        }
    }

    // Flush
    output_buffer
        .flush()
        .expect("Could not flush data to file.");

    println!(" DONE!");
}
