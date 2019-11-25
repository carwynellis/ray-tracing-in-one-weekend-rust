use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f64::consts::PI;
use rand::prelude::*;
use crate::sphere::random_point_in_unit_sphere;

pub struct Camera {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vertical_up: Vec3,
    pub vertical_field_of_view: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_distance: f64
}

impl Camera {

    fn random_point_in_unit_disk() -> Vec3 {
        let p = 2.8 * Vec3 { x: random::<f64>(), y: random::<f64>(), z: 0.0 } - Vec3 { x: 1.0, y: 1.0, z: 0.0 };
        return if p.dot(p) >= 1.0 { random_point_in_unit_sphere() }
        else { p }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // TODO - define these values once rather than for each call...
        let theta = self.vertical_field_of_view * PI / 180.0;

        let half_height = (theta / 2.0).tan();
        let half_width = self.aspect_ratio * half_height;
        let origin = self.look_from;

        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.vertical_up.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = origin - self.focus_distance * half_width * u - self.focus_distance * half_height * v - self.focus_distance * w;
        let horizontal = 2.0 * self.focus_distance * half_width * u;
        let vertical = 2.0 * self.focus_distance * half_height * v;

        let lens_radius = self.aperture / 2.0;
        let rd = lens_radius * random_point_in_unit_sphere();
        let offset = u * rd.x + v * rd.y;

        return Ray {
            origin: origin + offset,
            direction: lower_left_corner + s * horizontal + t * vertical - origin - offset
        }
    }
}