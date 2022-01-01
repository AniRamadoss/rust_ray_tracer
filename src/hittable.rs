use crate::ray::Ray;
use crate::Vec3;
use crate::vec3::Point3;


#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, front_face: bool) -> HitRecord {
        return HitRecord {
            p,
            normal,
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

pub trait Hittable<HitRecord>: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}