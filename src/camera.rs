use crate::vec3::Vec3;
use crate::ray::Ray;
use std::f64::consts::PI;
use rand::prelude::*;
use crate::hitable::sphere::random_point_in_unit_sphere;

pub struct Camera {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64
}

impl Camera {

    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vertical_up: Vec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let theta = vertical_field_of_view * PI / 180.0;

        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - look_at).unit_vector();
        let u = vertical_up.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = origin - focus_distance * half_width * u - focus_distance * half_height * v - focus_distance * w;
        let horizontal = 2.0 * focus_distance * half_width * u;
        let vertical = 2.0 * focus_distance * half_height * v;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            u,
            v,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius
        }
    }

    fn random_point_in_unit_disk(&self) -> Vec3 {
        let p = 2.8 * Vec3 { x: random::<f64>(), y: random::<f64>(), z: 0.0 } - Vec3 { x: 1.0, y: 1.0, z: 0.0 };
        return if p.dot(p) >= 1.0 { random_point_in_unit_sphere() }
        else { p }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * self.random_point_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset
        }
    }

}