use crate::math::{hit_sphere, lerp, Point3, Ray, Vec3,clamp,random_in_unit_sphere};
use crate::libcore::hit::Hittable;
use crate::libcore::hit::HitRecord::*;
use std::sync::Arc;


use std::io::{Error, Write};

pub type Color<T> = Vec3<T>;



pub fn ray_color(r: &Ray, world: & dyn Hittable,depth: usize) -> Color<f64> {
    if depth<=0{
        return Color::new();
    }

    match world.hit(r, 0.0, f64::MAX){
        Hit{normal,p,..} => {
            let target = p + normal + random_in_unit_sphere();
            return ray_color(&Ray::new(p, target-p), world,depth-1) * 0.5;
        },
        _ => ()
    };
    let unit_direction = r.direction.unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);

    lerp(
        &Color::with_values(1.0, 1.0, 1.0),
        &Color::with_values(0.5, 0.7, 1.0),
        t,
    )
}

pub fn write_color<W: Write>(f: &mut W, c: &Color<u8>) -> Result<(), Error> {
    f.write_fmt(format_args!("{:?} {:?} {:?}\n", c.x(), c.y(), c.z()))
}

pub fn transform_and_write_color<W: Write>(f: &mut W, c: &Color<f64>,samples_per_pixel: usize) -> Result<(), Error> {
    let c = transform_to_u8_color(c,samples_per_pixel);
    write_color(f, &c)
}

macro_rules! clamp_0_1 {
    ($t: expr) => {
        clamp($t,0.0,0.999)
    };
}

pub fn transform_to_u8_color(pixel_color: &Color<f64>,samples_per_pixel:usize ) -> Color<u8> {

    let scale = 1.0 / samples_per_pixel as f64;
    // With gamma=2.0
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();


    Color::with_values(
        (256.0 * clamp_0_1!(r)) as u8,
        (256.0 * clamp_0_1!(g)) as u8,
        (256.0 * clamp_0_1!(b)) as u8,
    )
}


