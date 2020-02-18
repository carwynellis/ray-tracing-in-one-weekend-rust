use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;

use rand::prelude::*;

use raytracer::camera::Camera;
use raytracer::hitable::Hitable;
use raytracer::hitable::hitable_list::HitableList;
use raytracer::ray::Ray;
use raytracer::scene::final_scene;
use raytracer::vec3::Vec3;

const MAXIMUM_RECURSION_DEPTH: i8 = 50;
const NEAR_ZERO: f64 = 0.001; // Treat hits that are less than this value as zero.

fn colour<T: Hitable>(r: Ray, world: &T, accumulator: Vec3, depth: i8) -> Vec3 {
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

fn main() -> std::io::Result<()> {
    let nx = 1200;
    let ny = 800;
    let samples = 1;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        20.0,
        nx as f64 / ny as f64,
        0.1,
        focus_distance
    );

    let world = HitableList {
        hitables: final_scene(),
    };

    let file_name = "image.ppm";

    let mut file = File::create(file_name)?;

    // Write PPM file header.
    file.write_fmt(format_args!("P3\n{}\n{}\n255\n", nx, ny))?;

    let max = 255.99;

    // Take writing to file off render path and do it at the end. Any faster?
    // TODO - refactor to map over a range so we don't need a mut vec here?
    let mut image_data = vec!();

    println!("Rendering scene to {}", file_name);
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Sample the pixel a number of times with a random offset and average the result to
            // antialias the overall image.
            let colour: Vec3 = (0..samples).map(|_| {
                let u = (i as f64 + random::<f64>()) / nx as f64;
                let v = (j as f64 + random::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                colour(r, &world, background_colour(&r), 0)
            }).fold(
                Vec3 { x: 0.0, y: 0.0, z: 0.0},
                |sum, v| sum + v
            ) / samples as f64;

            // Apply simple square root gamma correction to generated values.
            let gamma_corrected = Vec3::new(colour.x.sqrt(), colour.y.sqrt(), colour.z.sqrt());

            image_data.push(gamma_corrected);
        }
        let percent_complete = ((ny - j) as f64 / ny as f64) * 100.0;
        print!("\r{percent:>4}% complete ", percent = percent_complete.round());
        stdout().flush()?;
    }

    // TODO - look into buffered writers...
    // Build string first and then write to file....
    let mut formatted_data = "".to_string();

    image_data.into_iter().for_each(|pixel| {
        let line = format!("{} {} {}\n",
           (max * pixel.r()) as i64,
           (max * pixel.g()) as i64,
           (max * pixel.b()) as i64,
        );
        formatted_data.push_str(&line);
    });

    file.write_all(formatted_data.as_ref()).expect("Error writing to image file");

    println!("\nFinished");
    Ok(())
}
