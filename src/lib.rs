pub use self::{
    output::write_colour,
    ray::Ray,
    vector3::{
        util::{cross, dot, unit_vector},
        Colour, Point3, Vector3,
    },
};

pub mod output;
pub mod vector3;
pub mod ray;
