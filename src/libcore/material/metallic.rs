use super::Material;
use super::ScatterRecord;
use crate::color::Color;
use crate::libcore::hit::HitRecord;
use crate::math::{random_in_unit_sphere, reflect, Ray, Vec3};

pub struct Metallic {
    albedo: Color<f64>,
    fuzz: f64,
}

impl Material for Metallic {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&ray_in.direction.unit_vec(), &rec.normal);
        if reflected.dot(&rec.normal) > 0.0 {
            return Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz),
            });
        } else {
            None
        }
    }
}

impl Metallic {
    pub fn new(albedo: Color<f64>, fuzz: f64) -> Metallic {
        Metallic { albedo, fuzz }
    }
}
