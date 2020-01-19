use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::MaterialEnum;

use std::fmt::{Display, Formatter, Error};
use crate::hitable::sphere::Sphere;
use crate::hitable::hitable_list::HitableList;

pub mod hitable_list;
pub mod sphere;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: MaterialEnum
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return write!(f, "HitRecord(t: {}, p: {}, normal: {}", self.t, self.p, self.normal);
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

// TODO - like material, just trying this - better name?
#[derive(Clone)]
pub enum HitableEnum {
    Sphere(Sphere),
    HitableList(HitableList),
}

impl Hitable for HitableEnum {

    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        match *self {
            HitableEnum::Sphere(ref sphere) => sphere.hit(r, tmin, tmax),
            HitableEnum::HitableList(ref hitable_list) => hitable_list.hit(r, tmin, tmax),
        }
    }

}
