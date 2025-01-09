pub mod lambertian;
pub mod metal;

use crate::Ray;
use crate::Colour;
use crate::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool;
}