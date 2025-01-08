use raytracer::{camera::Camera, shapes::sphere::Sphere, HittableList, Point3};

fn main() {
    println!("Raytracer v0.1");

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 10;

    // Render!
    cam.render(&world);
}
