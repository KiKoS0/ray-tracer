
use crate::math::Vec3;
use std::io::{Error, Write};


pub type Color = Vec3<u8>;


pub fn write_color<W: Write>(f: &mut W,c: &Color) -> Result<(), Error>{
    f.write_fmt(format_args!("{:?} {:?} {:?}\n",c.x(),c.y(),c.z()))
}

pub fn transform_to_color(e : (f32,f32,f32)) -> Color {
    Color::with_values((255.9*e.0) as u8,(255.9*e.1) as u8,(255.9*e.2) as u8)
}