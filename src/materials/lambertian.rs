use crate::Material;
use crate::Colour;
use crate::Ray;
use crate::HitRecord;
use crate::Vector3;

pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vector3::random_unit();

        // Catch degenerate scatter direction.
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}