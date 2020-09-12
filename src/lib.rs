pub mod color;
pub mod libcore;
pub mod math;
pub mod utility;
use libcore::material::Dielectric;
use libcore::material::Lambertian;
use libcore::material::Material;
use libcore::material::Metallic;

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
use rand::Rng;

use std::time::Instant;

pub fn main() {
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

    let aspect_ratio = 3.0 / 2.0;
    let image_height = *image_width as f64 / aspect_ratio;
    let image_width = *image_width;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // Camera

    let lookfrom = Point3::with_values(13., 2., 3.);
    let lookat = Point3::with_values(0., 0., 0.);
    let vup = Vec3::with_values(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let vfov = 20.;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let thread_shared = ThreadData {
        camera: &cam,
        image_height: image_height as usize,
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
    };

    // World
    let ground_mat = Arc::new(Lambertian::new(Color::with_values(0.8, 0.8, 0.0)));
    let center_mat = Arc::new(Lambertian::new(Color::with_values(0.1, 0.2, 0.5)));
    let left_mat = Arc::new(Dielectric::new(1.5));
    let right_mat = Arc::new(Metallic::new(Color::with_values(0.8, 0.6, 0.2), 0.0));

    // let left_mat = Arc::new(Lambertian::new(Color::with_values(0., 0., 1.)));
    // let right_mat = Arc::new(Lambertian::new(Color::with_values(1., 0., 0.)));

    let mut world = HittableList::new();

    // world.add(Arc::new(Sphere::new(
    //     Point3::with_values(-R, 0., -1.0),
    //     R,
    //     left_mat.clone(),
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::with_values(R, 0., -1.0),
    //     R,
    //     right_mat.clone(),
    // )));

    world.add(Arc::new(Sphere::new(
        Point3::with_values(0.0, -100.5, -1.0),
        100.0,
        ground_mat.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(0.0, 0.0, -1.0),
        0.5,
        center_mat.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::with_values(-1.0, 0.0, -1.0),
        -0.45,
        left_mat.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(-1.0, 0.0, -1.0),
        0.5,
        left_mat.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::with_values(1.0, 0.0, -1.0),
        0.5,
        right_mat.clone(),
    )));

    // Render

    // let _ = func(&thread_shared, &out_file,Box::new(world));
    // let _ = generate_as_ppm(&thread_shared, &out_file, Arc::new(world));
    let world = random_scene();

    let now = Instant::now();

    match user_data {
        ImageFormat::PPM { .. } => {
            let _ = generate_as_ppm(&thread_shared, &out_file, &world);
        }
        ImageFormat::PNG { .. } => {
            let _ = generate_as_png(&thread_shared, &out_file, &world);
        }
        _ => panic!(),
    }
    let elapsed = now.elapsed();
    println!("Done. Elapsed: {:.2?}", elapsed);
}

pub fn generate_as_png<T: Hittable + Sync>(
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

            for _ in 0..data.samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / ((data.image_width - 1) as f64);
                let v = ((data.image_height - 1 - (j + top)) as f64 + random::<f64>())
                    / ((data.image_height - 1) as f64);

                let ray = data.camera.get_ray(u, v);
                pixel_color += &ray_color(&ray, world, data.max_depth);
            }

            pixels[j * bounds.1 + i] = transform_to_u8_color(&pixel_color, data.samples_per_pixel);
        }
    }
}

pub fn generate_as_ppm(data: &ThreadData, output: &String, world: &dyn Hittable) -> Result<()> {
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

                pixel_color += &ray_color(&ray, world, data.max_depth);
            }

            transform_and_write_color(&mut file, &pixel_color, data.samples_per_pixel)
                .expect("Error writing to stdout")
        }
    }
    return Ok(());
}

pub fn write_image_png(filename: &str, pixels: &[Color<u8>], bounds: (usize, usize)) -> Result<()> {
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

pub fn hello_world() {
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

fn random_scene() -> HittableList<Sphere> {
    let mut world = HittableList::new();
    let ground_mat = Arc::new(Lambertian::new(Color::with_values(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(0., -1000., 0.),
        1000.,
        ground_mat.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Point3::with_values(
                (a as f64) + 0.9 * random::<f64>(),
                0.2,
                (b as f64) + 0.9 * random::<f64>(),
            );
            if (center - Point3::with_values(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random::<Color<f64>>() * random::<Color<f64>>();
                    let sphere_mat = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat.clone())));
                } else if choose_mat < 0.95 {
                    // metal
                    let mut rng = rand::thread_rng();
                    let albedo = Color::with_values(
                        rng.gen_range(0., 0.5),
                        rng.gen_range(0., 0.5),
                        rng.gen_range(0., 0.5),
                    );
                    let fuzz = rng.gen_range(0., 0.5);
                    let sphere_mat = Arc::new(Metallic::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat.clone())));
                } else {
                    // glass
                    let sphere_mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat.clone())));
                }
            }
        }
    }
    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(0., 1., 0.),
        1.0,
        mat1.clone(),
    )));
    let mat2 = Arc::new(Lambertian::new(Color::with_values(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(-4., 1., 0.),
        1.0,
        mat2.clone(),
    )));
    let mat3 = Arc::new(Metallic::new(Color::with_values(0.7, 0.6, 0.5), 0.));
    world.add(Arc::new(Sphere::new(
        Point3::with_values(4., 1., 0.),
        1.0,
        mat3.clone(),
    )));

    return world;
}
