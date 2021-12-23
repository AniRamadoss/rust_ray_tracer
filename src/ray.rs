use crate::Vec3;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray {
            orig: origin,
            dir: direction,
        };
    }

    pub fn origin(&self) -> Point3 {
        return self.orig;
    }

    pub fn direction(&self) -> Point3 {
        return self.dir;
    }

    pub fn at(&self, t: f32) -> Point3 {
        return self.orig + t * self.dir;
    }
}