use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::hitable::_Hitable;

#[derive(Clone)]
pub struct HitableList {
    pub hitables: Vec<Hitable>
}

impl _Hitable for HitableList {
    // Note - this was implemented using fold, however the following runs around half the time.
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = tmax;

        for hitable in self.hitables.iter() {
            let hit = hitable.hit(r, tmin, closest_so_far);
            if hit.is_some() {
                result = hit;
                result.map(|h| closest_so_far = h.t);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;
    use crate::hitable::sphere::Sphere;
    use crate::material::lambertian::Lambertian;
    use crate::material::Material;
    use crate::hitable::Hitable;

    #[test]
    fn test_hit_returns_hit_record_if_one_of_the_objects_intersects_the_ray() {
        let sphere = Hitable::Sphere(Sphere {
            centre: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
            material: Material::Lambertian(Lambertian { albedo: Vec3 { x: 1.0, y: 1.0, z: 1.0 }}),
        });
        let ray = Ray {
            origin: Vec3 { x: 2.0, y: 2.0, z: 2.0 },
            direction: Vec3 { x: -2.0, y: -2.0, z: -2.0 },
        };
        let hitables = HitableList {
            hitables: vec![sphere]
        };
        let hit = hitables.hit(&ray, 0.0, 1.0);

        assert!(hit.is_some());
    }
}
