use std::ops::Neg;
use std::cmp::PartialEq;

// Class representing a point in three dimensional space.
// This is also used to store colour data during rendering.
#[derive(Debug)]
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
}

// Vec3 equality
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x &&
            self.y == other.y &&
            self.z == other.z;
    }
}

// Unary minus operator
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
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
    fn test_unary_minus_operator() {
        let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(-v, Vec3 { x: -1.0, y: -2.0, z: -3.0} )
    }

    #[test]
    fn test_squared_length() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(v.squared_length(), 29.0)
    }

}