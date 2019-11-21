use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::sphere::random_point_in_unit_sphere;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * v.dot(n) * n)
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray;
    fn albedo(&self) -> Vec3;
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray {
        let target = hit.p + hit.normal + random_point_in_unit_sphere();
        return Ray { origin: hit.p, direction: target - hit.p };
    }
    fn albedo(&self) -> Vec3 { self.albedo }
}

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
