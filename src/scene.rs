use crate::hitable::HitableEnum;
use rand::random;
use crate::vec3::Vec3;
use crate::hitable::sphere::Sphere;
use crate::material::MaterialEnum;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::dielectric::Dielectric;

pub fn final_scene() -> Vec<HitableEnum> {
    // Randomly generate a number of small spheres.
    let mut small_spheres: Vec<HitableEnum> = vec![];
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random::<f64>();
            let centre = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>()
            );
            if (centre - Vec3::new(4.0, 0.2, 0.0 )).length() > 0.9 {
                if choose_material < 0.8 {
                    // Create a diffuse sphere
                    small_spheres.push(HitableEnum::Sphere(Sphere {
                        centre,
                        radius: 0.2,
                        material: MaterialEnum::Lambertian(Lambertian {
                            albedo: Vec3::new(
                                random::<f64>() * random::<f64>(),
                                random::<f64>() * random::<f64>(),
                                random::<f64>() * random::<f64>(),
                            )
                        })
                    }))
                }
                else if choose_material < 0.95 {
                    // Create a metal sphere
                    small_spheres.push(HitableEnum::Sphere(Sphere {
                        centre,
                        radius: 0.2,
                        material: MaterialEnum::Metal(Metal {
                            albedo: Vec3::new(
                                0.5 * (1.0 + random::<f64>()),
                                0.5 * (1.0 + random::<f64>()),
                                0.5 * (1.0 + random::<f64>()),
                            ),
                            fuzziness: 0.5
                        })
                    }))
                }
                else {
                    // Create a glass sphere
                    small_spheres.push(HitableEnum::Sphere(Sphere {
                        centre,
                        radius: 0.2,
                        material: MaterialEnum::Dielectric(Dielectric { refractive_index: 1.5 })
                    }))
                }
            }
            else {  }
        }
    };

    let ground = HitableEnum::Sphere(Sphere { centre: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: MaterialEnum::Lambertian(Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5)}) });
    // Three more spheres that sit in the centre of the image.
    let glass_sphere = HitableEnum::Sphere(Sphere { centre: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: MaterialEnum::Dielectric(Dielectric { refractive_index: 1.5 }) });
    let matte_sphere = HitableEnum::Sphere(Sphere { centre: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: MaterialEnum::Lambertian(Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) }) });
    let metal_sphere = HitableEnum::Sphere(Sphere { centre: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: MaterialEnum::Metal(Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzziness: 0.0 }) });

    let all_spheres: Vec<HitableEnum> = vec![
        small_spheres,
        vec![ground, glass_sphere, matte_sphere, metal_sphere]
    ].into_iter().flatten().collect();

    return all_spheres;
}