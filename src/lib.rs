pub use self::{
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    materials::Material,
    materials::lambertian::Lambertian,
    materials::metal::Metal,
    output::write_colour,
    ray::Ray,
    shapes::sphere::Sphere,
    vector3::{
        util::{cross, dot, unit_vector},
        Colour, Point3, Vector3,
    },
};

pub mod camera;
pub mod hittable;
pub mod interval;
pub mod materials;
pub mod output;
pub mod ray;
pub mod shapes;
pub mod vector3;
