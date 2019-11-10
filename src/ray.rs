use crate::vec3::Vec3;

// Class representing a ray from a given origin that travels in a given direction.
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    // Returns a point on the ray starting at origin, in the given direction.
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + (t * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at_parameter() {
        let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let direction = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let ray = Ray { origin, direction };
        assert_eq!(ray.point_at_parameter(2.0), Vec3 { x: 2.0, y: 4.0, z: 6.0 })
    }
}
