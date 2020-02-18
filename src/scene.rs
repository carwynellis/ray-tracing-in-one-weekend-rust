use crate::hitable::HitableEnum;
use rand::random;
use crate::vec3::Vec3;
use crate::material::MaterialEnum;

pub fn final_scene() -> Vec<HitableEnum> {
    // TODO - can we collect instead of using a mutable vec?
    // Randomly generate a number of small spheres.
    let mut small_spheres: Vec<HitableEnum> = vec![];
    let radius = 0.2;
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random::<f64>();
            let centre = Vec3::new(
                a as f64 + 0.9 * random::<f64>(),
                radius,
                b as f64 + 0.9 * random::<f64>()
            );
            if (centre - Vec3::new(4.0, 0.2, 0.0 )).length() > 0.9 {
                if choose_material < 0.8 {
                    // Create a diffuse sphere
                    small_spheres.push(HitableEnum::sphere(
                        centre,
                        radius,
                        MaterialEnum::lambertian(
                            random::<f64>() * random::<f64>(),
                            random::<f64>() * random::<f64>(),
                            random::<f64>() * random::<f64>(),
                        )
                    ))
                }
                else if choose_material < 0.95 {
                    // Create a metal sphere
                    small_spheres.push(HitableEnum::sphere(
                        centre,
                        radius,
                        MaterialEnum::metal(
                            0.5 * (1.0 + random::<f64>()),
                            0.5 * (1.0 + random::<f64>()),
                            0.5 * (1.0 + random::<f64>()),
                            0.5,
                        )
                    ));
                }
                else {
                    // Create a glass sphere
                    small_spheres.push(HitableEnum::sphere(
                        centre,
                        radius,
                        MaterialEnum::dielectric(1.5)
                    ))
                }
            }
            else {  }
        }
    };

    let ground = HitableEnum::sphere(Vec3::new(0.0, -1000.0, 0.0), 1000.0, MaterialEnum::lambertian(0.5, 0.5, 0.5));
    // Three more spheres that sit in the centre of the image.
    let glass_sphere = HitableEnum::sphere(Vec3::new(0.0, 1.0, 0.0), 1.0, MaterialEnum::dielectric(1.5));
    let matte_sphere = HitableEnum::sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, MaterialEnum::lambertian(0.4, 0.2, 0.1));
    let metal_sphere = HitableEnum::sphere(Vec3::new(4.0, 1.0, 0.0), 1.0, MaterialEnum::metal(0.7, 0.6, 0.5, 0.0));

    let all_spheres: Vec<HitableEnum> = vec![
        small_spheres,
        vec![ground, glass_sphere, matte_sphere, metal_sphere]
    ].into_iter().flatten().collect();

    return all_spheres;
}