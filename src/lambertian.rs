use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use crate::material::Material;
use crate::ray::Ray;
use crate::{Color, Vec3};
use crate::vec3::Point3;
use crate::hittable::{Hittable, HitRecord};

pub struct Lambertian {
    pub(crate) albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        return Lambertian {
            albedo: a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let mut scattered = Ray::new(rec.p, scatter_direction);
        return Some((self.albedo, scattered));
    }

    fn clone(&self) -> Rc<dyn Material> {
        return Rc::new(Lambertian {
            albedo: self.albedo,
        });
    }
}