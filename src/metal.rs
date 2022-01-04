use crate::material::Material;
use crate::ray::Ray;
use crate::{Color, Vec3};
use crate::vec3::Point3;
use crate::hittable::{Hittable, HitRecord};

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(a: &Color) -> Metal {
        return Metal {
            albedo: *a,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, mut attenuation: Box<Color>, mut scattered: Box<Ray>) -> bool {
        let reflected: Vec3 = Vec3::reflect(&(Vec3::unit_vector(r_in.direction())), &rec.normal);
        scattered = Box::new((Ray::new(rec.p, reflected)));
        attenuation = Box::new((self.albedo.clone()));
        return scattered.direction().dot(rec.normal) > 0.0;
    }

    fn clone(&self) -> Box<dyn Material> {
        return Box::new(Metal {
            albedo: self.albedo,
        });
    }
}
