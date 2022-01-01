use crate::hittable;
use std::rc::Rc;
use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct hittable_list {
    objects: Vec<Rc<HitRecord>>,
}

pub trait HittableList<hittable_list> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

impl hittable_list {

    pub fn new(list: Vec<Rc<HitRecord>>) -> hittable_list {
        return hittable_list {
            objects: list,
        };
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<HitRecord>) {
        self.objects.push(object);
    }
}

impl HittableList<hittable_list> for hittable_list {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for obj in self.objects {
            if let Some(hit) = Rc::try_unwrap(obj).unwrap().hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = closest_so_far;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }
        return hit_anything;

    }
}