use crate::{
    unit_vector, write_colour, Colour, HitRecord, Hittable, Interval, Point3, Ray, Vector3,
};
use rand::Rng;
use std::fs::File;
use std::io::{stdout, BufWriter, Write};

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render<T: Hittable>(&mut self, world: &T) {
        self.initialize();

        // Open output file for writing.
        let output_filename = "./output/pretty.ppm";
        let output_file = File::create(output_filename)
            .unwrap_or_else(|_| panic!("Could not open output file: {output_filename}"));
        let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);

        // Header
        let _bytes_written = output_buffer
            .write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .unwrap();

        print!("Rendering");

        // Render loop.
        for j in 0..self.image_height {
            print!(".");
            stdout().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_colour = Colour::default();

                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.max_depth, world)
                }

                let final_colour = self.pixel_samples_scale * pixel_colour;
                write_colour(&mut output_buffer, &final_colour);
            }
        }

        // Flush
        output_buffer
            .flush()
            .expect("Could not flush data to file.");

        println!(" DONE!");
    }

    fn initialize(&mut self) {
        self.image_height = u32::max((self.image_width as f64 / self.aspect_ratio) as u32, 1);
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = Point3::default();

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors for pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel_origin = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    ///
    /// Casts a ray out into the world and returns the computed.  It recursively calls itself until
    /// the depth is reduced to 0, averaging the colour returned from each bounce.
    ///
    fn ray_colour<T: Hittable>(&self, r: &Ray, depth: u32, world: &T) -> Colour {
        if depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(r, Interval::build(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vector3::random_unit();
            return 0.5 * self.ray_colour(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction = unit_vector(r.direction());
        let colour_one = Colour::new(1.0, 1.0, 1.0);
        let colour_two = Colour::new(0.5, 0.7, 1.0);
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * colour_one + a * colour_two
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_origin
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            center: Default::default(),
            pixel_origin: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            pixel_samples_scale: Default::default(),
        }
    }
}
