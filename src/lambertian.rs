use crate::material::Material;
use crate::ray::Ray;
use crate::{Color, Vec3};
use crate::vec3::Point3;
use crate::hittable::{Hittable, HitRecord};

pub struct Lambertian {
    pub(crate) albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Lambertian {
        return Lambertian {
            albedo: *a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, mut attenuation: Box<Color>, mut scattered: Box<Ray>) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        scattered = Box::new((Ray::new(rec.p, scatter_direction)));
        let mut col = Box::new(self.albedo.clone());
        attenuation = col;
        return true;
    }

    fn clone(&self) -> Box<dyn Material> {
        return Box::new(Lambertian {
            albedo: self.albedo,
        });
    }
}