use crate::math::{hit_sphere, lerp_vector, Point3, Ray, Vec3};
use std::io::{Error, Write};

pub type Color<T> = Vec3<T>;

pub fn ray_color(r: &Ray) -> Color<f64> {
    let sphere_center = Point3::with_values(0.0, 0.0, -1.0);
    let t = hit_sphere(&sphere_center, 0.5, r);
    if let Some(res) = t {
        let N = (r.at(res) - sphere_center).unit_vec();
        return Color::with_values(1.0 + N.x(), 1.0 + N.y(), 1.0 + N.z()) * 0.5;
    }
    let unit_direction = r.direction.unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // eprintln!("\nray: {:?}\nunit: {:?}\nt: {:?}",r.direction,unit_direction,t);
    // let _ = std::io::stderr().flush();
    Color::with_values(1.0, 1.0, 1.0) * (1.0 - t) + Color::with_values(0.5, 0.7, 1.0) * t;
    lerp_vector(
        &Color::with_values(1.0, 1.0, 1.0),
        &Color::with_values(0.5, 0.7, 1.0),
        t,
    )
}

pub fn write_color<W: Write>(f: &mut W, c: &Color<u8>) -> Result<(), Error> {
    f.write_fmt(format_args!("{:?} {:?} {:?}\n", c.x(), c.y(), c.z()))
}

pub fn transform_and_write_color<W: Write>(f: &mut W, c: &Color<f64>) -> Result<(), Error> {
    let c = transform_to_u8_color((c.x(), c.y(), c.z()));
    write_color(f, &c)
}

pub fn transform_to_u8_color(e: (f64, f64, f64)) -> Color<u8> {
    Color::with_values(
        (255.9 * e.0) as u8,
        (255.9 * e.1) as u8,
        (255.9 * e.2) as u8,
    )
}
