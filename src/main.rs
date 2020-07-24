mod color;
mod libcore;
mod math;
mod utility;
use color::{ray_color, transform_and_write_color, transform_to_u8_color, write_color, Color};
use libcore::camera::Camera;
use libcore::hit::Hittable;
use libcore::hittable_list::HittableList;
use math::sphere::Sphere;
use math::Point3;
use math::Ray;
use math::Vec3;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use utility::{parse, ImageFormat, ThreadData};

extern crate image;
use image::png::PNGEncoder;
use image::ColorType;
extern crate rayon;
use rayon::prelude::*;
extern crate rand;
use rand::random;

fn main() {
    let mut image_width;
    let mut out_file;
    let mut format;

    let user_data = parse();

    match &user_data {
        ImageFormat::PNG { width, filename } => {
            image_width = width;
            out_file = filename;
            format = "png".to_string();
        }
        ImageFormat::PPM { width, filename } => {
            image_width = width;
            out_file = filename;
            format = "ppm".to_string();
        }
        ImageFormat::Unknown => {
            writeln!(std::io::stderr(), "Unknown image file format");
            panic!();
        }
    }

    let aspect_ratio = 16.0 / 9.0;
    let image_height = *image_width as f64 / aspect_ratio;
    let image_width = *image_width;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera

    let cam = Camera::new();

    let thread_shared = ThreadData {
        camera: &cam,
        image_height: image_height as usize,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth
    };

    // World

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::with_values(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(0.0, -100.5, -1.0),
        100.0,
    )));

    // Render

    // let _ = func(&thread_shared, &out_file,Box::new(world));
    // let _ = generate_as_ppm(&thread_shared, &out_file, Arc::new(world));

    match user_data {
        ImageFormat::PPM { .. } => {
            let _ = generate_as_ppm(&thread_shared, &out_file, &world);
        }
        ImageFormat::PNG { .. } => {
            let _ = generate_as_png(&thread_shared, &out_file, &world);
        }
        _ => panic!(),
    }
    println!("Done.");
}

fn generate_as_png<T: Hittable + Sync>(
    data: &ThreadData,
    output: &String,
    world: &T,
) -> Result<()> {
    let image_width = data.image_width;
    let image_height = data.image_height;

    let mut pixels = vec![Color::new(); image_width * image_height as usize];
    println!(
        "width: {:?} height: {:?} pixels: {:?}",
        image_width,
        image_height,
        image_height as usize * image_width
    );

    // Old Crossbeam version horizontal bands
    // let threads = num_cpus::get();
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
        render(band, band_bounds, top, data, world);
    });

    write_image_png(output, &mut pixels, (image_width, image_height as usize))
}

fn render(
    pixels: &mut [Color<u8>],
    bounds: (usize, usize),
    top: usize,
    data: &ThreadData,
    world: &dyn Hittable,
) {
    for j in 0..bounds.1 {
        for i in 0..bounds.0 {
            let mut pixel_color = Color::new();

            for s in 0..data.samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / ((data.image_width - 1) as f64);
                let v = ((data.image_height - 1 - (j + top)) as f64 + random::<f64>())
                    / ((data.image_height - 1) as f64);

                let ray = data.camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, world,data.max_depth);
            }

            pixels[j * bounds.1 + i] = transform_to_u8_color(&pixel_color, data.samples_per_pixel);
        }
    }
}

fn generate_as_ppm(data: &ThreadData, output: &String, world: &dyn Hittable) -> Result<()> {
    let mut file = match File::create(output) {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    let aspect_ratio = data.aspect_ratio;
    let image_width = data.image_width;
    let image_height = image_width as f64 / aspect_ratio;

    // println!("P3\n{:?} {:?}\n255", image_width, image_height as u32);
    file.write_fmt(format_args!(
        "P3\n{:?} {:?}\n255\n",
        image_width, image_height as u32
    ))
    .expect("Unable to write data");

    for i in (0..(image_height as u64)).rev() {
        write!(std::io::stderr(), "\rScanlines remaining: {:?} ", i);
        for j in 0..image_width {
            let mut pixel_color = Color::<f64>::new();
            for s in 0..data.samples_per_pixel {
                let u = (j as f64 + random::<f64>()) / ((image_width - 1) as f64);
                let v = (i as f64 + random::<f64>()) / (image_height - 1.0);
                let ray = data.camera.get_ray(u, v);

                pixel_color += &ray_color(&ray, world,data.max_depth);
            }

            transform_and_write_color(&mut file, &pixel_color, data.samples_per_pixel)
                .expect("Error writing to stdout")
        }
    }
    return Ok(());
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
    // let pixels: Vec<&Color<u8>> = pixels.into_iter().rev().collect();
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

fn hello_world() {
    let args: Vec<String> = std::env::args().collect();

    let (width, height): (usize, usize) =
        parse_pair(&args[1], 'x').expect("error parsing image dimensions");

    println!("P3\n{:?} {:?}\n255", width, height);
    for i in (0..height).rev() {
        write!(std::io::stderr(), "\rScanlines remaining: {:?} ", i);
        for j in 0..width {
            let c = transform_to_u8_color(
                &Color::with_values(
                    (j as f64) / (width as f64 - 1.),
                    (i as f64) / (height as f64 - 1.),
                    0.25,
                ),
                100,
            );
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
