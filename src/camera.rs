use crate::{Point3, Ray, Vec3};
use crate::rtweekend::degrees_to_radians;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
        const FOCAL_LENGTH: f32 = 1.0;

        // Vertical FOV in Deg
        let theta = degrees_to_radians(vfov as f64);
        let viewport_height = 2.0 * ((theta as f32) / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let h = viewport_width * u;
        let v = viewport_height * v;
        let llc = lookfrom - h / 2.0 - v / 2.0 - w;

        // let aspect_ratio: f32 = 16.0 / 9.0;
        // let viewport_height: f32 = 2.0;
        // let viewport_width: f32 = aspect_ratio * viewport_height;
        // let focal_length: f32 = 1.0;

        return Camera {
            origin: lookfrom,
            lower_left_corner: llc,
            horizontal: h,
            vertical: v,
        };
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin);
    }
}