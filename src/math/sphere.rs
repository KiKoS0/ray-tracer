use crate::libcore::hit::HitRecord;
use crate::libcore::hit::HitRecord::*;
use crate::libcore::hit::Hittable;
use super::Point3;
use super::Ray;
use super::Vec3;
use std::sync::Arc;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let temp = discriminant.sqrt();

            let root = (-half_b - temp) / a;
            if root < t_max && root > t_min {
                let p = ray.at(root);
                return HitRecord::new_hit(p, root, ray, &((p - self.center) / self.radius));
            }
            let root = (-half_b + temp) / a;
            if root < t_max && root > t_min {
                let p = ray.at(root);
                return HitRecord::new_hit(p, root, ray, &((p - self.center) / self.radius));
            }
        };
        Miss
    }
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Sphere {
            center,
            radius,
        }
    }
}
