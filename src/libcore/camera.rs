use crate::math::Point3;
use crate::math::Ray;
use crate::math::Vec3;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Point3::with_values(0.0, 0.0, 0.0);
        let horizontal = Vec3::with_values(viewport_width, 0.0, 0.0);
        let vertical = Vec3::with_values(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::with_values(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
