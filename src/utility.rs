use std::str::FromStr;
use crate::libcore::camera::Camera;

extern crate clap;
use clap::{App, Arg};

#[derive(Clone, Copy)]
pub struct ThreadData<'a> {
    pub camera: &'a Camera,
    pub aspect_ratio: f64,
    pub image_height: usize,
    pub image_width: usize,
    pub samples_per_pixel:usize,
    pub max_depth: usize
}

pub enum ImageFormat {
    PNG { width: usize, filename: String },
    PPM { width: usize, filename: String },
    Unknown,
}

pub fn parse() -> ImageFormat {
    let matches = App::new("Ray Tracer Test")
        .version("0.1.0")
        .author("KiKoS")
        .about("Ray tracer generating ppm and png files")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Output file format"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                // .required(true)
                .help("Width of the image"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("Output file name"),
        )
        .get_matches();

    let format = matches.value_of("format").unwrap_or("png");
    let output = matches.value_of("output").unwrap_or("img.png");
    let width = usize::from_str(matches.value_of("width").unwrap_or("400"))
        .expect("Width cannot be parsed! Should be a number");

    match format {
        "png" => ImageFormat::PNG {
            width,
            filename: output.to_string(),
        },
        "ppm" => ImageFormat::PPM {
            width,
            filename: output.to_string(),
        },
        _ => ImageFormat::Unknown,
    }
}