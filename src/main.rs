mod color;
mod math;
mod utility;
use color::ray_color;
use color::transform_and_write_color;
use color::transform_to_u8_color;
use color::write_color;
use color::Color;
use math::Point3;
use math::Ray;
use math::Vec3;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::str::FromStr;
use utility::{parse, ImageFormat, ThreadData};

extern crate image;
use image::png::PNGEncoder;
use image::ColorType;
extern crate num_cpus;
extern crate rayon;
use rayon::prelude::*;

fn main() {
    let mut image_width;
    let mut out_file;
    let mut func: fn(&ThreadData, &String) -> Result<()>;

    match parse() {
        ImageFormat::PNG { width, filename } => {
            image_width = width;
            out_file = filename;
            func = generate_as_png;
        }
        ImageFormat::PPM { width, filename } => {
            image_width = width;
            out_file = filename;
            func = generate_as_ppm;
        }
        ImageFormat::Unknown => {
            writeln!(std::io::stderr(), "Unknown image file format");
            panic!();
        }
    }

    let aspect_ratio = 16.0 / 9.0;
    let image_height = image_width as f64 / aspect_ratio;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::with_values(0.0, 0.0, 0.0);
    let horizontal = Vec3::with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::with_values(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::with_values(0.0, 0.0, focal_length);

    let thread_shared = ThreadData {
        horizontal: &horizontal,
        vertical: &vertical,
        image_height: image_height as usize,
        image_width,
        origin: &origin,
        lower_left_corner: &lower_left_corner,
        aspect_ratio,
    };

    let _ = func(&thread_shared, &out_file);
    println!("Done.");
}

fn write_image_png(filename: &str, pixels: &[Color<u8>], bounds: (usize, usize)) -> Result<()> {
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    let encoder = PNGEncoder::new(output);
    // Flattening a big array takes a lot of time
    // TODO: Optimize this by making the render function
    // directly produce the rgb array
    let pixels: Vec<u8> = pixels
        .iter()
        .flat_map(|s| s.as_std_vec().into_iter())
        .collect();
    match encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8) {
        Ok(_) => Ok(()),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not encode png",
        )),
    }
}

fn render(pixels: &mut [Color<u8>], bounds: (usize, usize), top: usize, data: ThreadData) {
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            // println!("x: {:?}  y: {:?}", row + top, column);
            // std::io::stdout().flush();
            let u = column as f64 / ((data.image_width - 1) as f64);
            let v = (row + top) as f64 / ((data.image_height - 1) as f64);
            let ray = Ray::new(
                *data.origin,
                (*data.lower_left_corner) + (*data.horizontal) * u + (*data.vertical) * v
                    - (*data.origin),
            );
            let pixel_color = ray_color(&ray);
            pixels[row * bounds.0 + column] =
                transform_to_u8_color((pixel_color.x(), pixel_color.y(), pixel_color.z()));
        }
    }
}

fn generate_as_png(data: &ThreadData, output: &String) -> Result<()> {
    let image_width = data.image_width;
    let image_height = data.image_height;

    let mut pixels = vec![Color::new(); image_width * image_height as usize];
    println!(
        "width: {:?} height: {:?} pixels: {:?}",
        image_width,
        image_height,
        image_height as usize * image_width
    );
    let threads = num_cpus::get();

    // Old Crossbeam version horizontal bands
    // let rows_per_band = image_height as usize / threads + 1;
    // {
    //     let bands: Vec<&mut [Color<u8>]> = pixels.chunks_mut(rows_per_band * image_width).collect();
    //     let _ = crossbeam::scope(|spawner| {
    //         for (i, band) in bands.into_iter().enumerate() {
    //             let top = rows_per_band * i;
    //             let height = band.len() / image_width;
    //             let band_bounds = (image_width, height);
    //             spawner.spawn(move |_| {
    //                 render(band, band_bounds, top, *data);
    //             });
    //         }
    //     });
    // }

    let bands: Vec<(usize, &mut [Color<u8>])> =
        pixels.chunks_mut(image_width).enumerate().collect();
    bands.into_par_iter().for_each(|(i, band)| {
        let top = i;
        let band_bounds = (image_width, 1);
        render(band, band_bounds, top, *data);
    });

    write_image_png(output, &mut pixels, (image_width, image_height as usize))
}

fn generate_as_ppm(data: &ThreadData, output: &String) -> Result<()> {
    let mut file = match File::create(output) {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    let aspect_ratio = data.aspect_ratio;
    let image_width = data.image_width;
    let image_height = image_width as f64 / aspect_ratio;

    let origin = data.origin;
    let horizontal = data.horizontal;
    let vertical = data.vertical;
    let lower_left_corner = data.lower_left_corner;

    // println!("P3\n{:?} {:?}\n255", image_width, image_height as u32);
    file.write_fmt(format_args!(
        "P3\n{:?} {:?}\n255\n",
        image_width, image_height as u32
    ))
    .expect("Unable to write data");

    for i in (0..(image_height as u64)).rev() {
        write!(std::io::stderr(), "\rScanlines remaining: {:?} ", i);
        for j in 0..image_width {
            let u = j as f64 / ((image_width - 1) as f64);
            let v = i as f64 / (image_height - 1.0);

            let ray = Ray::new(
                *origin,
                (*lower_left_corner) + (*horizontal) * u + (*vertical) * v - (*origin),
            );
            let pixel_color = ray_color(&ray);
            transform_and_write_color(&mut file, &pixel_color).expect("Error writing to stdout")
        }
    }
    write!(std::io::stderr(), "\nDone.\n");
    return Ok(());
}

fn hello_world() {
    let args: Vec<String> = std::env::args().collect();

    let (width, height): (usize, usize) =
        parse_pair(&args[1], 'x').expect("error parsing image dimensions");

    println!("P3\n{:?} {:?}\n255", width, height);
    for i in (0..height).rev() {
        write!(std::io::stderr(), "\rScanlines remaining: {:?} ", i);
        for j in 0..width {
            let c = transform_to_u8_color((
                (j as f64) / (width as f64 - 1.),
                (i as f64) / (height as f64 - 1.),
                0.25,
            ));
            let _ = write_color(&mut std::io::stdout(), &c);
        }
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}
