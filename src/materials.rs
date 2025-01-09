pub mod lambertian;

use crate::Ray;
use crate::Colour;
use crate::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool;
}