use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::hitable::Hitable;

// TODO - review lifetime parameters. Are they correct here?
#[derive(Clone)]
pub struct HitableList<'a> {
    pub hitables: Vec<&'a dyn Hitable>
}

impl Hitable for HitableList<'_> {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        // TODO - try this without any local mutable state.
        let mut result = None;
        let mut closest_so_far = tmax;

        for hitable in self.hitables.iter() {
            let hit = hitable.hit(r, tmin, closest_so_far);
            match hit {
                Some(ref h) => {
                    result = hit;
                    closest_so_far = h.t;
                }
                None => ()
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_returns_hit_record_if_one_of_the_objects_intersects_the_ray() {
        let sphere = Sphere {
            centre: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 2.0, y: 2.0, z: 2.0 },
            direction: Vec3 { x: -2.0, y: -2.0, z: -2.0 },
        };
        let hitables = HitableList {
            hitables: vec![&sphere]
        };
        let hit = hitables.hit(ray, 0.0, 1.0);

        assert!(hit.is_some());
    }
}
