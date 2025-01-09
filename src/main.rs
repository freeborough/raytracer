use std::sync::Arc;
use raytracer::{camera::Camera, shapes::sphere::Sphere, Colour, HittableList, Lambertian, Metal, Point3};

fn main() {
    println!("Raytracer v0.1");

    // Materials
    let material_ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Colour::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2));

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Arc::new(material_ground)));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Arc::new(material_center)));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Arc::new(material_left)));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Arc::new(material_right)));


    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1024;
    cam.samples_per_pixel = 50;

    // Render!
    cam.render(&world);
}
