use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

// Determine if ray intersects a sphere. This will be the case where the discriminant is greater
// than zero, which indicates if there are one or two real solutions to the quadratic equation
// that describes the intersection of a ray with a sphere.
fn hit_sphere(centre: Vec3, radius: f64, r: Ray) -> bool {
    let oc = r.origin - centre;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);
    return discriminant > 0.0;
}

// Compute a linear blend between white and blue depending on the value of the y coordinate.
// Show intersection of ray with a sphere.
fn colour(r: Ray) -> Vec3 {
    // If the ray intersects the sphere, return red, otherwise fallback to the gradient.
    if hit_sphere(Vec3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, r ) {
        return Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    }
    else {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    }
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
