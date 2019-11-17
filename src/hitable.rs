use crate::vec3::Vec3;
use crate::ray::Ray;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return write!(f, "HitRecord(t: {}, p: {}, normal: {}", self.t, self.p, self.normal);
    }
}

pub trait Hitable {
    fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

