use std::rc::Rc;
use crate::ray::Ray;
use crate::{Metal, Vec3};
use crate::vec3::Point3;
use crate::material::Material;
use crate::vec3::Color;

// #[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, front_face: bool) -> HitRecord {
        let color = Rc::new(Metal::new(Color::new(0.0, 0.0, 0.0), 0.0));
        return HitRecord {
            p,
            normal,
            mat_ptr: color,
            t,
            front_face,
        }
    }
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-1.0 * (*outward_normal)};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}