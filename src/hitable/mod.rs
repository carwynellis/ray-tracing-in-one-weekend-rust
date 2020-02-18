use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

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
    pub material: Material
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return write!(f, "HitRecord(t: {}, p: {}, normal: {}", self.t, self.p, self.normal);
    }
}

trait _Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub enum Hitable {
    Sphere(Sphere),
    HitableList(HitableList),
}

// Provide constructors for available hitables to clean up the API.
impl Hitable {

    pub fn sphere(centre: Vec3, radius: f64, material: Material) -> Hitable {
        Hitable::Sphere(Sphere { centre, radius, material })
    }

    pub fn hitable_list(hitables: Vec<Hitable>) -> Hitable {
        Hitable::HitableList(HitableList { hitables })
    }

    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        match *self {
            Hitable::Sphere(ref sphere) => sphere.hit(r, tmin, tmax),
            Hitable::HitableList(ref hitable_list) => hitable_list.hit(r, tmin, tmax),
        }
    }

}
