use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::material::lambertian::Lambertian;
use crate::material::dielectric::Dielectric;
use crate::material::metal::Metal;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * v.dot(n) * n)
}

// TODO - make this private?
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray;
    fn albedo(&self) -> Vec3;
}

// TODO - just trying this - better name?
#[derive(Copy, Clone)]
pub enum MaterialEnum {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MaterialEnum {

    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray {
        match *self {
            MaterialEnum::Lambertian(ref lambertian) => lambertian.scatter(ray_in, hit),
            MaterialEnum::Metal(ref metal) => metal.scatter(ray_in, hit),
            MaterialEnum::Dielectric(ref dielectric) => dielectric.scatter(ray_in, hit),
        }
    }

    fn albedo(&self) -> Vec3 {
        match *self {
            MaterialEnum::Lambertian(ref lambertian) => lambertian.albedo(),
            MaterialEnum::Metal(ref metal) => metal.albedo(),
            MaterialEnum::Dielectric(ref dielectric) => dielectric.albedo(),
        }
    }

}

impl MaterialEnum {

    pub fn dielectric(refractive_index: f64) -> MaterialEnum {
        return MaterialEnum::Dielectric(Dielectric { refractive_index });
    }

    pub fn lambertian(r: f64, g: f64, b: f64) -> MaterialEnum {
        return MaterialEnum::Lambertian(Lambertian {
            albedo: Vec3::new(r, g, b)
        })
    }

    pub fn metal(r: f64, g: f64, b: f64, fuzziness: f64) -> MaterialEnum {
        return MaterialEnum::Metal(Metal {
            albedo: Vec3::new(r, g, b),
            fuzziness
        })
    }

}
