pub mod ray;
pub mod sphere;
pub mod vec3;

pub use self::ray::Ray;
pub use self::vec3::Vec3;
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