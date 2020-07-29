use crate::color::Color;
use crate::libcore::hit::HitRecord;
use crate::math::ray::Ray;


pub mod lambertian;
pub mod metallic;


pub use lambertian::Lambertian;
pub use metallic::Metallic;


pub struct ScatterRecord {
    pub attenuation: Color<f64>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}
