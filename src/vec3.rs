use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

// Class representing a point in three dimensional space.
// This is also used to store colour data during rendering.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    // Alias x, y, z since we also use this class to store colour data.
    pub fn r(&self) -> f64 { self.x }
    pub fn g(&self) -> f64 { self.y }
    pub fn b(&self) -> f64 { self.z }

    pub fn squared_length(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    // Divide this vector by its length to generate a unit vector that has length 1.
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    // Compute the dot product of this and another Vec3
    pub fn dot(&self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // Compute the cross product of this and another Vec3
    // Implementation verified with https://betterexplained.com/articles/cross-product/
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: -((self.x * other.z) - (self.z * other.x)),
            z: (self.x * other.y) - (self.y * other.x)
        }
    }
}

// Vec3 equality
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.z == other.z
    }
}

// Unary minus operator
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

// Add operator
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colour_aliases() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.r(), v.x);
        assert_eq!(v.g(), v.y);
        assert_eq!(v.b(), v.z);
    }

    #[test]
    fn test_equality_returns_true_for_two_equal_vectors() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v1 == v2, true)
    }

    #[test]
    fn test_equality_returns_false_for_two_unequal_vectors() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(v1 == v2, false)
    }

    #[test]
    fn test_unary_minus_operator() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(-v, Vec3 { x: -1.0, y: -2.0, z: -3.0 })
    }

    #[test]
    fn test_squared_length() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(v.squared_length(), 29.0)
    }

    #[test]
    fn test_length() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        let result: f64 = 29.0;
        assert_eq!(v.length(), result.sqrt())
    }

    #[test]
    fn test_add_operator() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1 + v2, Vec3 { x: 5.0, y: 7.0, z: 9.0 })
    }

    #[test]
    fn test_sub_operator() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v2 - v1, Vec3 { x: 3.0, y: 3.0, z: 3.0 })
    }

    #[test]
    fn test_multiply_by_vec3_operator() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 2.0, y: 2.0, z: 2.0 };
        assert_eq!(v1 * v2, Vec3 { x: 2.0, y: 4.0, z: 6.0 })
    }

    #[test]
    fn test_multiply_vec3_by_f64_operator() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v * 0.5, Vec3 { x: 0.5, y: 1.0, z: 1.5 })
    }

    #[test]
    fn test_multiply_f64_by_vec3_operator() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(0.5 * v, Vec3 { x: 0.5, y: 1.0, z: 1.5 })
    }

    #[test]
    fn test_divide_by_vec3_operator() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 2.0, y: 2.0, z: 2.0 };
        assert_eq!(v1 / v2, Vec3 { x: 0.5, y: 1.0, z: 1.5 })
    }

    #[test]
    fn test_divide_by_f64_operator() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v / 2.0, Vec3 { x: 0.5, y: 1.0, z: 1.5 })
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        // Total length of unit vector should be one.
        assert_eq!(v.unit_vector().length(), 1.0)
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 2.0, y: 2.0, z: 2.0 };
        assert_eq!(v1.dot(v2), 12.0)
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1.cross(v2), Vec3 { x: -3.0, y: 6.0, z: -3.0 })
    }
}
