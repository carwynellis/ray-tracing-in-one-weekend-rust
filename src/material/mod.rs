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

// Internal trait that defines the API for underlying Materials.
// Note that the Material enum forms the public API for materials and wraps these private types.
trait _Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray;
    fn albedo(&self) -> Vec3;
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {

    pub fn dielectric(refractive_index: f64) -> Material {
        return Material::Dielectric(Dielectric { refractive_index });
    }

    pub fn lambertian(r: f64, g: f64, b: f64) -> Material {
        return Material::Lambertian(Lambertian {
            albedo: Vec3::new(r, g, b)
        })
    }

    pub fn metal(r: f64, g: f64, b: f64, fuzziness: f64) -> Material {
        return Material::Metal(Metal {
            albedo: Vec3::new(r, g, b),
            fuzziness
        })
    }

    pub fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray {
        match *self {
            Material::Lambertian(ref lambertian) => lambertian.scatter(ray_in, hit),
            Material::Metal(ref metal) => metal.scatter(ray_in, hit),
            Material::Dielectric(ref dielectric) => dielectric.scatter(ray_in, hit),
        }
    }

    pub fn albedo(&self) -> Vec3 {
        match *self {
            Material::Lambertian(ref lambertian) => lambertian.albedo(),
            Material::Metal(ref metal) => metal.albedo(),
            Material::Dielectric(ref dielectric) => dielectric.albedo(),
        }
    }

}
