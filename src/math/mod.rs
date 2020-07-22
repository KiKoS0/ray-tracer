pub mod ray;
pub mod vec3;

pub use self::ray::Ray;
pub use self::vec3::Vec3;

pub type Point3<T> = Vec3<T>;

pub fn lerp_vector(a: &Vec3<f64>, b: &Vec3<f64>, t: f64) -> Vec3<f64> {
    (*a) * (1.0 - t) + (*b) * t
}


pub fn hit_sphere(center : &Point3<f64>,radius: f64, ray: &Ray) -> Option<f64>{
    let oc = ray.origin - (*center);
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c; 
    match discriminant {
        x if x < 0.0 => None,
        _ => Some((-b - discriminant.sqrt())/ (2.0* a))
    }
}