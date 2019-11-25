use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f64::consts::PI;

pub struct Camera {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vertical_up: Vec3,
    pub vertical_field_of_view: f64,
    pub aspect_ratio: f64
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // TODO - define these values once rather than for each call...
        let theta = self.vertical_field_of_view * PI / 180.0;

        let half_height = (theta / 2.0).tan();
        let half_width = self.aspect_ratio * half_height;
        let origin = self.look_from;

        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.vertical_up.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        return Ray {
            origin,
            direction: lower_left_corner + s * horizontal + t * vertical - origin
        }
    }
}