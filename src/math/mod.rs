pub mod ray;
pub mod sphere;
pub mod vec3;

pub use self::ray::Ray;
pub use self::vec3::Vec3;
use super::rand::random;
use core::ops::Add;
use core::ops::Mul;

extern crate rand_distr;
use rand::Rng;
use rand_distr::{Distribution, UnitBall};

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

/// This method uses normally distributed random numbers technique
/// suggested here
/// https://math.stackexchange.com/questions/87230/picking-random-points-in-the-volume-of-sphere-with-uniform-probability/87238#87238
#[inline]
pub fn random_in_unit_sphere() -> Vec3<f64> {
    let u = random::<f64>();
    let mut v = random::<Vec3<i32>>();

    let c = u.cbrt();
    (Vec3::with_values(v.x() as f64, v.y() as f64, v.z() as f64)).unit_vec() * c
}

/// This method applies the rejection method
#[inline]
pub fn random_in_unit_sphere_rejection() -> Vec3<f64> {
    let v: [f64; 3] = UnitBall.sample(&mut rand::thread_rng());
    Vec3::with_values(v[0], v[1], v[2])
}

/// This method applies the rejection method
#[inline]
pub fn random_in_hemisphere(normal: &Vec3<f64>) -> Vec3<f64> {
    let v = random_in_unit_sphere();
    if v.dot(normal) > 0.0 {
        v
    } else {
        -v
    }
}

/// This method applies a lambertian distribution
/// (Creating a true lambertian diffuse surface)
#[inline]
pub fn random_unit_vector() -> Vec3<f64> {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z = rng.gen_range(-1.0, 1.0);
    let r = (1f64 - z * z).sqrt();
    Vec3::with_values(r * a.cos(), r * a.sin(), z)
}

#[inline]
pub fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64> {
    (*v) - (*n) * v.dot(n) * 2.0
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
