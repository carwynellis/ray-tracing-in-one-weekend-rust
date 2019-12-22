use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * v.dot(n) * n)
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray;
    fn albedo(&self) -> Vec3;
}
