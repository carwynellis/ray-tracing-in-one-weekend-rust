use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::hitable::Hitable;

#[derive(Clone)]
pub struct HitableList<'a> {
    pub hitables: Vec<&'a dyn Hitable>
}

impl Hitable for HitableList<'_> {
    // Note - this was implemented using fold, however the following runs around half the time.
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = tmax;

        for hitable in self.hitables.iter() {
            let hit = hitable.hit(r, tmin, closest_so_far);
            if hit.is_some() {
                result = hit;
                &result.map(|h| closest_so_far = h.t);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;
    use crate::sphere::Sphere;
    use crate::material::Lambertian;

    #[test]
    fn test_hit_returns_hit_record_if_one_of_the_objects_intersects_the_ray() {
        let sphere = Sphere {
            centre: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
            material: Box::new(Lambertian { albedo: Vec3 { x: 1.0, y: 1.0, z: 1.0 }}),
        };
        let ray = Ray {
            origin: Vec3 { x: 2.0, y: 2.0, z: 2.0 },
            direction: Vec3 { x: -2.0, y: -2.0, z: -2.0 },
        };
        let hitables = HitableList {
            hitables: vec![&sphere]
        };
        let hit = hitables.hit(&ray, 0.0, 1.0);

        assert!(hit.is_some());
    }
}
