use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use crate::material::Material;
use crate::ray::Ray;
use crate::{Color, Vec3};
use crate::vec3::Point3;
use crate::hittable::{Hittable, HitRecord};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Metal {
        let mut fz = f;
        if f >= 1.0 {
            fz = 1.0;
        }
        return Metal {
            albedo: a,
            fuzz: fz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected: Vec3 = Vec3::reflect(&(Vec3::unit_vector(r_in.direction())), &rec.normal);
        let mut scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        }
        else {
            None
        }
    }

    fn clone(&self) -> Rc<dyn Material> {
        return Rc::new(Metal {
            albedo: self.albedo,
            fuzz: self.fuzz,
        });
    }
}
