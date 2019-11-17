use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}