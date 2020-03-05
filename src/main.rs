use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, BufWriter};
use std::sync::atomic::{AtomicI64, Ordering};

use rand::prelude::*;
use rayon::prelude::*;

use raytracer::camera::Camera;
use raytracer::hitable::Hitable;
use raytracer::ray::Ray;
use raytracer::scene::final_scene;
use raytracer::vec3::Vec3;

const MAXIMUM_RECURSION_DEPTH: i8 = 50;
const NEAR_ZERO: f64 = 0.001; // Treat hits that are less than this value as zero.

const WIDTH: i64 = 1200; // Image width - pixels
const HEIGHT: i64 = 800; // Image height - pixels
const SAMPLES: i64 = 10; // Samples per pixel

fn colour(r: Ray, world: &Hitable, accumulator: Vec3, depth: i8) -> Vec3 {
    match world.hit(&r, NEAR_ZERO, std::f64::MAX) {
        Some(ref hit) if depth < MAXIMUM_RECURSION_DEPTH => {
            let scattered = hit.material.scatter(&r, &hit);
            colour(scattered, world, hit.material.albedo() * accumulator, depth + 1)
        }
        _ => accumulator
    }
}

// Compute a linear blend between white and blue depending on the value of the y coordinate.
fn background_colour(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn render_pixel(x: i64, y: i64, world: &Hitable, camera: &Camera) -> Vec3 {
    // Sample the pixel a number of times with a random offset and average the result to
    // antialias the overall image.
    let colour: Vec3 = (0..SAMPLES).map(|_| {
        let u = (x as f64 + random::<f64>()) / WIDTH as f64;
        let v = (y as f64 + random::<f64>()) / HEIGHT as f64;
        let r = camera.get_ray(u, v);
        colour(r, world, background_colour(&r), 0)
    }).fold(
        Vec3 { x: 0.0, y: 0.0, z: 0.0},
        |sum, v| sum + v
    ) / SAMPLES as f64;

    // Apply simple square root gamma correction to generated values.
    return Vec3::new(colour.x.sqrt(), colour.y.sqrt(), colour.z.sqrt());
}

fn main() -> std::io::Result<()> {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        20.0,
        WIDTH as f64 / HEIGHT as f64,
        0.1,
        focus_distance
    );

    let world = Hitable::hitable_list(final_scene());

    let file_name = "image.png";

    println!("Rendering scene to {}", file_name);

    static PROGRESS: AtomicI64 = AtomicI64::new(0);

    let image_data: Vec<Vec<Vec3>> = (0..HEIGHT).into_par_iter().map(|j| {
        let line =  (0..WIDTH).map(|i| return render_pixel(i, j, &world, &camera)).collect();
        PROGRESS.fetch_add(1, Ordering::Relaxed);
        let percent_complete = (PROGRESS.load(Ordering::Relaxed) as f64 / HEIGHT as f64) * 100.0;
        print!("\r{percent:>4}% complete ", percent = percent_complete.round());
        stdout().flush().expect("failed to flush stdout");
        return line;
    }).collect();
    
    let png_file = File::create(file_name).expect("Unable to open PNG file for writing");
    let w = BufWriter::new(png_file);
    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);

    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);

    let mut png_writer = encoder.write_header().unwrap();

    // TODO - better expressed as an extension method?
    fn component_value(v: f64) -> u8 { (v * 255.99) as u8 }

    // Convert image data into RGBA
    let rgba_data: Vec<Vec<u8>> = image_data.into_iter().rev().flatten().map(|pixel| {
        return vec!(
            component_value(pixel.r()),
            component_value(pixel.g()),
            component_value(pixel.b()),
            255,
        );
    }).collect();

    let png_data: Vec<u8> = rgba_data.into_iter().flatten().collect();

    png_writer.write_image_data(&png_data).expect("Failed to write image data");

    println!("\nFinished");

    Ok(())
}
