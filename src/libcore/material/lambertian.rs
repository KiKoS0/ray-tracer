use super::Material;
use super::ScatterRecord;
use crate::color::Color;
use crate::libcore::hit::HitRecord;
use crate::math::{Vec3,Ray,random_unit_vector};

pub struct Lambertian {
    albedo: Color<f64>,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_dir = rec.normal + random_unit_vector();
        Some(ScatterRecord{
            attenuation:self.albedo,
            scattered: Ray::new(rec.p,scatter_dir)
        })

    }
}

impl Lambertian {
    pub fn new(albedo: Color<f64>) -> Lambertian {
        Lambertian { albedo }
    }
}
