use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use std::io::stdout;

// TODO - tidy up modules
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;
mod material;

use vec3::Vec3;
use ray::Ray;
use hitable_list::HitableList;
use sphere::Sphere;
use sphere::random_point_in_unit_sphere;
use hitable::Hitable;
use camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};
use std::f64::consts::PI;
use std::borrow::Borrow;

const MAXIMUM_RECURSION_DEPTH: i8 = 50;
const NEAR_ZERO: f64 = 0.001; // Treat hits that are less than this value as zero.

fn colour<T: Hitable>(r: Ray, world: &T, accumulator: Vec3, depth: i8) -> Vec3 {
    match world.hit(r, NEAR_ZERO, std::f64::MAX) {
        Some(hit) if depth < MAXIMUM_RECURSION_DEPTH => {
            let scattered = hit.material.scatter(&r, &hit);
            return colour(scattered, world, hit.material.albedo() * accumulator, depth + 1)
        }
        _ => return accumulator
    }
}

// Compute a linear blend between white and blue depending on the value of the y coordinate.
fn background_colour(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 }
}

//fn random_scene() -> Vec<&'static Sphere<'static>> {
//    // Randomly generate a number of small spheres.
//    let small_spheres: Vec<&Sphere> = (-11..11).map(|a| {
//        (-11..11).map(|b| {
//            let choose_material = random::<f64>();
//            let centre = Vec3 {
//                x: a as f64 + 0.9 * random::<f64>(),
//                y: 0.2,
//                z: b as f64 + 0.9 * random::<f64>()
//            };
//            if (centre - Vec3 { x: 4.0, y: 0.2, z: 0.0 }).length() > 0.9 {
//               if choose_material < 0.8 {
//                   // Create a diffuse sphere
//                   Some(&Sphere {
//                       centre,
//                       radius: 0.2,
//                       material: &Lambertian {
//                           albedo: Vec3 {
//                               x: random::<f64>() * random::<f64>(),
//                               y: random::<f64>() * random::<f64>(),
//                               z: random::<f64>() * random::<f64>(),
//                           }
//                        }
//                    })
//                }
//                else if choose_material < 0.95 {
//                    // Create a metal sphere
//                    Some(&Sphere {
//                        centre,
//                        radius: 0.2,
//                        material: &Metal {
//                            albedo: Vec3 {
//                                x: 0.5 * (1.0 + random::<f64>()),
//                                y: 0.5 * (1.0 + random::<f64>()),
//                                z: 0.5 * (1.0 + random::<f64>()),
//                            },
//                            fuzziness: 0.5 * random::<f64>()
//                        }
//                    })
//                }
//                else {
//                    // Create a glass sphere
//                    Some(&Sphere {
//                        centre,
//                        radius: 0.2,
//                        material: &Dielectric { refractive_index: 1.5}
//                    })
//                }
//            }
//            else { None }
//        }).flatten()
//    }).flatten().collect();

//    let ground = Sphere { centre: Vec3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: &Lambertian { albedo: Vec3 { x: 0.5, y: 0.5, z: 0.5 }}};
//    // Three more spheres that sit in the centre of the image.
//    let glass_sphere = &Sphere { centre: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 1.0, material: &Dielectric { refractive_index: 1.5 }};
//    let matte_sphere = &Sphere { centre: Vec3 { x: -4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: &Lambertian { albedo: Vec3 { x: 0.4, y: 0.2, z: 0.1 } }};
//    let metal_sphere = &Sphere { centre: Vec3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: &Metal { albedo: Vec3 { x: 0.7, y: 0.6, z: 0.5 }, fuzziness: 0.0 }};

//    let all_spheres = vec![
//        small_spheres,
//        vec![&ground, &glass_sphere, &matte_sphere, &metal_sphere]
//        ].into_iter().flatten().collect();

//    return all_spheres;
//}

fn main() -> std::io::Result<()> {
    let nx = 1200;
    let ny = 800;
    let samples = 100;

    let look_from = Vec3 { x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let focus_distance = 10.0;

    let camera = Camera {
        look_from: look_from,
        look_at: look_at,
        vertical_up: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        vertical_field_of_view: 20.0,
        aspect_ratio: nx as f64 / ny as f64,
        aperture: 0.1,
        focus_distance: focus_distance
    };

//    let world = HitableList {
//        hitables: vec![
//            &Sphere { centre: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5, material: &Lambertian { albedo: Vec3 { x: 0.1, y: 0.2, z: 0.5 }} },
//            &Sphere { centre: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0, material: &Lambertian { albedo: Vec3 { x: 0.8, y: 0.8, z: 0.0 }} },
//            &Sphere { centre: Vec3 { x: 1.0, y: 0.0, z: -1.0 }, radius: 0.5, material: &Metal { albedo: Vec3 { x: 0.8, y: 0.6, z: 0.2 }, fuzziness: 1.0 } },
//            &Sphere { centre: Vec3 { x: -1.0, y: 0.0, z: -1.0 }, radius: 0.5, material: &Dielectric { refractive_index: 1.5} },
//        ]
//    };

//    let r = (PI / 4.0).cos();

//    let s1 = Sphere { centre: Vec3 { x: -r, y: 0.0, z: -1.0 }, radius: r, material: &Lambertian { albedo: Vec3 { x: 0.0, y: 0.0, z: 1.0 }} };
//    let s2 = Sphere { centre: Vec3 { x: r, y: 0.0, z: -1.0 }, radius: r, material: &Lambertian { albedo: Vec3 { x: 1.0, y: 0.0, z: 0.0 }} };
//    let spheres = vec![&s1, &s2].into_iter().map(|s| s as &dyn Hitable).collect();

//    let world = HitableList {
//        hitables: spheres,
//    };

    // Initial attempt at rendering just the main static spheres.
    // TODO - resolve the reference lifetime fun and add the random spheres to the vector too.
    let ground = Sphere { centre: Vec3 { x: 0.0, y: -1000.0, z: 0.0 }, radius: 1000.0, material: &Lambertian { albedo: Vec3 { x: 0.5, y: 0.5, z: 0.5 }}};
    // Three more spheres that sit in the centre of the image.
    let glass_sphere = Sphere { centre: Vec3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 1.0, material: &Dielectric { refractive_index: 1.5 }};
    let matte_sphere = Sphere { centre: Vec3 { x: -4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: &Lambertian { albedo: Vec3 { x: 0.4, y: 0.2, z: 0.1 } }};
    let metal_sphere = Sphere { centre: Vec3 { x: 4.0, y: 1.0, z: 0.0 }, radius: 1.0, material: &Metal { albedo: Vec3 { x: 0.7, y: 0.6, z: 0.5 }, fuzziness: 0.0 }};

    let all_spheres = vec![&ground, &glass_sphere, &matte_sphere, &metal_sphere].into_iter().map(|s| s as &dyn Hitable).collect();

    let world = HitableList {
        hitables: all_spheres
    };

    let file_name = "image.ppm";

    let mut file = File::create(file_name)?;

    // Write PPM file header.
    file.write_fmt(format_args!("P3\n{}\n{}\n255\n", nx, ny))?;

    let max = 255.99;

    println!("Rendering image...");
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
            let gamma_corrected = Vec3 {
                x: colour.x.sqrt(),
                y: colour.y.sqrt(),
                z: colour.z.sqrt(),
            };

            file.write_fmt(format_args!("{} {} {}\n",
                (max * gamma_corrected.r()) as i64,
                (max * gamma_corrected.g()) as i64,
                (max * gamma_corrected.b()) as i64,
            ))?;
        }
        // TODO - tidy this up
        print!("Completed line {} of {}             \r", ny - j, ny);
        stdout().flush()?;
    }

    println!("Finished.");
    Ok(())
}
