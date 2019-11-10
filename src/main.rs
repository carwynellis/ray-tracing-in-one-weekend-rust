use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

// Compute a linear blend between white and blue depending on the value of the y coordinate.
fn colour(r: Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 };
}

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vec3 { x: -2.0, y: -1.0, z: -1.0 };
    let horizontal = Vec3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    let file_name = "image.ppm";

    let mut file = File::create(file_name)?;

    // Write PPM file header.
    file.write_fmt(format_args!("P3\n{}\n{}\n255\n", nx, ny))?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray {
                origin,
                direction: lower_left_corner + (u * horizontal) + (v * vertical)
            };
            let colour = colour(r);
            let max = 255.99;
            file.write_fmt(format_args!("{} {} {}\n",
                (max * colour.r()) as i64,
                (max * colour.g()) as i64,
                (max * colour.b()) as i64,
            ))?;
        }
    }

    Ok(())
}
