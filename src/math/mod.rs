pub mod ray;
pub mod sphere;
pub mod vec3;

pub use self::ray::Ray;
pub use self::vec3::Vec3;
use super::rand::random;
use core::ops::Add;
use core::ops::Mul;

pub type Point3<T> = Vec3<T>;

pub fn hit_sphere(center: &Point3<f64>, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - (*center);
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    match discriminant {
        x if x < 0.0 => None,
        _ => Some((-half_b - discriminant.sqrt()) / a),
    }
}

#[inline]
pub fn degrees_to_radian(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    match value {
        x if x < min => min,
        x if x > max => max,
        _ => value,
    }
}

#[inline]
pub fn lerp<T: Copy + Add<Output = T> + Mul<f64, Output = T>>(a: &T, b: &T, t: f64) -> T {
    (*a) * (1.0 - t) + (*b) * t
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3<f64> {
    let u = random::<f64>();
    let mut v = random::<Vec3<i32>>();

    let c = u.cbrt();
    (Vec3::with_values(v.x() as f64, v.y() as f64, v.z() as f64)).unit_vec() * c
}

mod tests {
    use super::random_in_unit_sphere;
    use std::io::Write;
    #[test]
    fn random_point() {
        // let mut output = match std::fs::File::create("nums.log") {
        //     Ok(f) => f,
        //     Err(err) => panic!(),
        // };
        for i in 0..10000 {
            let v = random_in_unit_sphere();
            assert_eq!(true, v.length() < 1.0);
            // output
            //     .write_fmt(format_args!("{:?}\n", v))
            //     .expect("Unable to write data");
        }
    }
}
