use crate::ray::Ray;
use crate::Vec3;
use crate::vec3::Point3;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;

// #[derive(Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    mat_ptr: Box<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f32, m: Box<dyn Material>) -> Sphere {
        return Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //Nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        return true;
    }
}