use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

use std::fmt::{Display, Formatter, Error};

pub mod hitable_list;
pub mod sphere;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material
}

impl Display for HitRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return write!(f, "HitRecord(t: {}, p: {}, normal: {}", self.t, self.p, self.normal);
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

