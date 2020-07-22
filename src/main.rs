use std::io::Write;
mod color;
mod math;
use color::ray_color;
use color::transform_and_write_color;
use color::transform_to_u8_color;
use color::write_color;
use math::Point3;
use math::Ray;
use math::Vec3;
use std::str::FromStr;

fn main() {
    let image_width = 400;
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

    println!("P3\n{:?} {:?}\n255", image_width, image_height as u32);
    for i in (0..(image_height as u64)).rev() {
        write!(std::io::stderr(), "\rScanlines remaining: {:?} ", i);
        // eprintln!("=========Starting {:?}=========", i);
        // let _ = std::io::stderr().flush();

        for j in 0..image_width {
            let u = j as f64 / ((image_width - 1) as f64);
            let v = i as f64 / (image_height - 1.0);

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&ray);
            transform_and_write_color(&mut std::io::stdout(), &pixel_color)
                .expect("Error writing to stdout")
        }
    }
    write!(std::io::stderr(), "\nDone.\n");
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
