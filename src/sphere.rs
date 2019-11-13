use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::hitable::Hitable;

#[derive(Debug)]
struct Sphere {
    pub centre: Vec3,
    pub radius: f64
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = r.direction.dot(r.direction);
        let b =  oc.dot(r.direction);
        let c = oc.dot(oc) - (self.radius.powi(2));
        let discriminant = (b * b) - (a * c);

        // If discriminant is zero we have one or two real solutions to the quadratic equation that
        // describes the intersection of the ray with the sphere.
        if discriminant > 0.0 {
            let solution1 = (-b - discriminant.sqrt()) / a;
            if solution1 < tmax && solution1 > tmin {
                let intersection_point = r.point_at_parameter(solution1);
                let hit_record = HitRecord {
                   t: solution1,
                   p: intersection_point,
                   normal: (intersection_point - self.centre) / self.radius,
                };
                return Some(hit_record);
            }

            let solution2 = (-b + discriminant.sqrt()) / a;
            if solution2 < tmax && solution2 > tmin {
                let intersection_point = r.point_at_parameter(solution1);
                let hit_record = HitRecord {
                    t: solution1,
                    p: intersection_point,
                    normal: (intersection_point - self.centre) / self.radius,
                };
                return Some(hit_record);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_returns_hit_record_if_ray_intersects_sphere() {
        let sphere = Sphere {
            centre: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 2.0, y: 2.0, z: 2.0 },
            direction: Vec3 { x: -2.0, y: -2.0, z: -2.0 },
        };
        let hit = sphere.hit(ray, 0.0, 1.0);

        assert_eq!(hit.is_some(), true);
    }

    #[test]
    fn test_hit_returns_none_if_ray_does_not_intersect_sphere() {
        let sphere = Sphere {
            centre: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
        };
        let ray = Ray {
            origin: Vec3 { x: 2.0, y: 2.0, z: 2.0 },
            direction: Vec3 { x: 2.0, y: 2.0, z: 2.0 },
        };
        let hit = sphere.hit(ray, 0.0, 1.0);

        // TODO - do we need a PartialEq impl on HitRecord instead?
        assert_eq!(hit.is_none(), true);
    }
}
