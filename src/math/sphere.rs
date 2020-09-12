use std::sync::Arc;
use super::Point3;
use super::Ray;
use super::Vec3;
use crate::libcore::hit::HitRecord;
use crate::libcore::hit::Hittable;
use crate::libcore::material::Material;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // This is meant as an optimization
        // Cloning an Arc a lot of time can be cpu consuming
        // So instead just share an unsafe reference and make sure
        // that HitRecords don't outlive the Spheres arcs
        let mat_ptr = Arc::as_ptr(&self.material);

        if discriminant > 0.0 {
            let temp = discriminant.sqrt();

            let root = (-half_b - temp) / a;
            if root < t_max && root > t_min {
                let p = ray.at(root);
                return Some(HitRecord::new_hit(
                    p,
                    root,
                    ray,
                    &((p - self.center) / self.radius),
                    unsafe{ &*mat_ptr },
                ));
            }
            let root = (-half_b + temp) / a;
            if root < t_max && root > t_min {
                let p = ray.at(root);
                return Some(HitRecord::new_hit(
                    p,
                    root,
                    ray,
                    &((p - self.center) / self.radius),
                    unsafe{ &*mat_ptr },
                ));
            }
        };
        None
    }
}

impl Sphere {
    pub fn new(
        center: Point3<f64>,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
