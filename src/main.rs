use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

// Determine if ray intersects a sphere. This will be the case where the discriminant is greater
// than zero, which indicates if there are one or two real solutions to the quadratic equation
// that describes the intersection of a ray with a sphere.
fn hit_sphere(centre: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - centre;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        return -1.0;
    }
    else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

// Compute a linear blend between white and blue depending on the value of the y coordinate.
// Show intersection of ray with a sphere and map the surface normal to a colour.
fn colour(r: Ray) -> Vec3 {
    let t =  hit_sphere(Vec3 { x: 0.0, y: 0.0, z: -1.0 }, 0.5, r);
    // If we hit the sphere, compute the surface normal and use this to determine the pixel colour.
    if t > 0.0 {
        let normal = (r.point_at_parameter(t) - Vec3 { x: 0.0, y: 0.0, z: -1.0 }).unit_vector();
        return 0.5 * Vec3 {
            x: normal.x + 1.0,
            y: normal.y + 1.0,
            z: normal.z + 1.0
        }
    }
    else {
        // Ray intersects nothing so return gradient.
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    }
}

fn main() -> std::io::Result<()> {
    let nx = 800;
    let ny = 400;

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
