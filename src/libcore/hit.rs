use crate::libcore::material::Material;
use crate::math::Point3;
use crate::math::Ray;
use crate::math::Vec3;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

impl HitRecord {
    // pub fn set_face_normal(self, r: &Ray, outward_normal: &Vec3<f64>) -> Self {
    //     match self {
    //         HitRecord::Hit {
    //             front_face, normal, ..
    //         } => {
    //             front_face = r.direction.dot(outward_normal) < 0.0;
    //             normal = if front_face {
    //                 *outward_normal
    //             } else {
    //                 -(*outward_normal)
    //             }
    //         }
    //         _ => panic!("Cannot set face normal to a ray miss"),
    //     };
    //     self
    // }

    pub fn new_hit(
        p: Point3<f64>,
        t: f64,
        r: &Ray,
        outward_normal: &Vec3<f64>,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
        HitRecord {
            p,
            t,
            normal,
            front_face,
            material,
        }
    }
}
