use super::Point3;
use super::Vec3;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }

    #[inline(never)]
    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}
