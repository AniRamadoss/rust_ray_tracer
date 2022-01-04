use crate::ray::Ray;
use crate::{Color, Vec3};
use crate::vec3::Point3;
use crate::hittable::{Hittable, HitRecord};

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: Box<Color>, scattered: Box<Ray>) -> bool;
    fn clone(&self) -> Box<dyn Material>;
}