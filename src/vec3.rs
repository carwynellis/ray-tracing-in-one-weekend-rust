// Class representing a point in three dimensional space.
// This is also used to store colour data during rendering.

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
}