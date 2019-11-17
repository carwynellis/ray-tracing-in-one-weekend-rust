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

//// Attempt to allow Clone of Hitable.
//// TODO - what other, perhaps better approaches are there?
//trait HitableClone {
//    fn hitable_clone(&self) -> Box<dyn Hitable>;
//}

//impl <T> HitableClone for T where T: Hitable + Clone {
//    fn hitable_clone(&self) -> Box<dyn Hitable> {
//        Box::new(self.clone())
//    }
//}
