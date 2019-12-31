use crate::vec3::Vec3;
use crate::material::{Material, reflect};
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::hitable::sphere::random_point_in_unit_sphere;

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzziness: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray {
        let reflected = reflect(ray_in.direction.unit_vector(), hit.normal);
        return Ray { origin: hit.p, direction: reflected + self.fuzziness * random_point_in_unit_sphere()};
    }
    fn albedo(&self) -> Vec3 { self.albedo }
}
