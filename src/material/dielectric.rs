use crate::vec3::Vec3;
use crate::material::{Material, reflect};
use crate::ray::Ray;
use crate::hitable::HitRecord;
use rand::random;

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refractive_index: f64
}

impl Dielectric {
    pub fn refract(&self, v: Vec3, n: Vec3, ni_over_nt: f64) -> Vec3 {
        let unit_vector_of_v = v.unit_vector();
        let dt = unit_vector_of_v.dot(n);
        let discriminant = 1.0 - (ni_over_nt * ni_over_nt * ( 1.0 - (dt * dt)));
        return if discriminant > 0.0 { (ni_over_nt * (unit_vector_of_v - (n * dt))) - (n * discriminant.sqrt()) }
        else { v };
    }

    // Polynomial approximation of glass reflectivity.
    pub fn schlick(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refractive_index) / (1.0 + self.refractive_index);
        let r0_squared = r0 * r0;
        return r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Ray {
        let reflected = reflect(ray_in.direction.unit_vector(), hit.normal);
        let (outward_normal, ni_over_nt, cosine) = if ray_in.direction.dot(hit.normal) > 0.0 {
            (-hit.normal, self.refractive_index, self.refractive_index * ray_in.direction.dot(hit.normal) / ray_in.direction.length())
        }
        else {
            (hit.normal, 1.0 / self.refractive_index, -ray_in.direction.dot(hit.normal) / ray_in.direction.length())
        };

        let refracted = self.refract(ray_in.direction, outward_normal, ni_over_nt);

        let reflection_probability = if refracted == ray_in.direction { 1.0 }
        else  { self.schlick(cosine) };

        return if random::<f64>() < reflection_probability { Ray { origin: hit.p, direction: reflected } }
        else { Ray { origin: hit.p, direction: refracted } };
    }

    fn albedo(&self) -> Vec3 { Vec3 { x: 1.0, y: 1.0, z: 1.0} }
}
