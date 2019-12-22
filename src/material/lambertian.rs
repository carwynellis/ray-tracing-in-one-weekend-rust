use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::hitable::sphere::random_point_in_unit_sphere;

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Ray {
        let target = hit.p + hit.normal + random_point_in_unit_sphere();
        return Ray { origin: hit.p, direction: target - hit.p };
    }
    fn albedo(&self) -> Vec3 { self.albedo }
}
