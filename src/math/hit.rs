use super::Point3;
use super::Ray;
use super::Vec3;

pub enum HitRecord {
    Hit {
        p: Point3<f64>,
        normal: Vec3<f64>,
        t: f64,
        front_face: bool,
    },
    Miss,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitRecord;
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

    pub fn new_hit(p: Point3<f64>, t: f64, r: &Ray, outward_normal: &Vec3<f64>) -> HitRecord {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
        HitRecord::Hit {
            p,
            t,
            normal,
            front_face,
        }
    }
}
