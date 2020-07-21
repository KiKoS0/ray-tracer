use std::io::Write;
mod math;
mod color;
use math::Vec3;
use color::transform_to_color;
use color::write_color;
use std::str::FromStr;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (width,height): (usize, usize) = parse_pair(&args[1], 'x').expect("error parsing image dimensions");


    println!("P3\n{:?} {:?}\n255",width,height);
    for i in (0..height).rev(){
        write!(std::io::stderr(),"\rScanlines remaining: {:?} ",i);
        for j in 0..width {
            let c =  transform_to_color(((j as f32) / (width as f32-1.),(i as f32) / (height as f32-1.),0.25));
            let _ = write_color(&mut std::io::stdout(),&c);
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