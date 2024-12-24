use raytracer::{unit_vector, write_colour, dot, Colour, Point3, Ray, Vector3};
use std::fs::File;
use std::io::{BufWriter, Write};

fn hit_sphere(centre: Point3, radius: f64, r: &Ray) -> bool {
    let oc = centre - *r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2.0 * dot(r.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant: f64 = b*b - 4.0 * a * c;

    discriminant >= 0.0
}

fn ray_colour(r: &Ray) -> Colour {
    if hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Colour::new_colour(1.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let colour_one = Colour::new(1.0, 1.0, 1.0);
    let colour_two = Colour::new(0.5, 0.7, 1.0);
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * colour_one + a * colour_two
}

fn main() {
    println!("Raytracer v0.1");

    // Image - Derrive height from width and aspect ration, and make sure it's at least 1 pixel high.
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = u32::max((image_width as f64 / aspect_ratio) as u32, 1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors for pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    // Open output file for writing.
    let output_filename = "./output.ppm";
    let output_file = File::create(output_filename)
        .unwrap_or_else(|_| panic!("Could not open output file: {output_filename}"));
    let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);

    // Header
    let _bytes_written = output_buffer
        .write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    print!("Rendering");

    // Render loop.
    for j in 0..image_height {
        print!(".");

        for i in 0..image_width {
            let pixel_centre =
                pixel_origin + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_colour = ray_colour(&r);
            write_colour(&mut output_buffer, &pixel_colour);
        }
    }

    // Flush
    output_buffer
        .flush()
        .expect("Could not flush data to file.");

    println!(" DONE!");
}
