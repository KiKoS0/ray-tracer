use super::Material;
use super::ScatterRecord;
use crate::color::Color;
use crate::libcore::hit::HitRecord;
use crate::math::{random_unit_vector, Ray, Vec3};

pub struct Lambertian {
    albedo: Color<f64>,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_dir = rec.normal + random_unit_vector();
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_dir),
        })
    }
}

impl Lambertian {
    pub fn new(albedo: Color<f64>) -> Lambertian {
        Lambertian { albedo }
    }
}
