use std::rc::Rc;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;
use crate::vec3::Color;

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        return Dielectric {
            index_of_refraction: ir,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f32 = if rec.front_face {1.0 / self.index_of_refraction} else { self.index_of_refraction };
        let unit_direction = Vec3::unit_vector(r_in.direction());
        let refracted = unit_direction.refract(rec.normal, refraction_ratio);
        let scattered = Ray::new(rec.p, refracted);
        return Some((Color::new(1.0, 1.0, 1.0), scattered));
    }

    fn clone(&self) -> Rc<dyn Material> {
        return Rc::new(Dielectric::new(self.index_of_refraction.clone()));
    }
}