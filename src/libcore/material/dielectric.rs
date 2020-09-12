use super::Material;
use super::ScatterRecord;
use crate::color::Color;
use crate::libcore::hit::HitRecord;
use crate::math::{random_in_unit_sphere, reflect, refract, schlick, Ray, Vec3};
use rand::random;

pub struct Dielectric {
    ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::with_values(1.0, 1.0, 1.0);
        let etai_over_eta = match rec.front_face {
            true => 1.0 / self.ref_idx,
            false => self.ref_idx,
        };
        let unit_direction = ray_in.direction.unit_vec();

        let cos_theta = f64::min(1.0, rec.normal.dot(-unit_direction));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let scattered;
        if etai_over_eta * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &rec.normal);
            scattered = Ray::new(rec.p, reflected);
            return Some(ScatterRecord {
                attenuation,
                scattered,
            });
        }
        let reflect_prob = schlick(cos_theta, etai_over_eta);
        if random::<f64>() < reflect_prob {
            let reflected = reflect(&unit_direction, &rec.normal);
            scattered = Ray::new(rec.p, reflected);
            return Some(ScatterRecord {
                attenuation,
                scattered,
            });
        }

        let refracted = refract(&unit_direction, &rec.normal, etai_over_eta);
        scattered = Ray::new(rec.p, refracted);

        return Some(ScatterRecord {
            attenuation,
            scattered,
        });
    }
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}
