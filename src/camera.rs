use crate::{Point3, Ray, rtweekend, Vec3};
use crate::rtweekend::degrees_to_radians;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera {

        // Vertical FOV in Deg
        let theta = (rtweekend::PI as f32) / 180.0 * vfov;
        let viewport_height = 2.0 * ((theta as f32) / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let _w = Vec3::unit_vector(lookfrom - lookat);
        let _u = Vec3::unit_vector(Vec3::cross(vup, _w));
        let _v = Vec3::cross(_w, _u);

        let h = focus_dist * viewport_width * _u;
        let v = focus_dist * viewport_height * _v;
        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * _w;

        // let aspect_ratio: f32 = 16.0 / 9.0;
        // let viewport_height: f32 = 2.0;
        // let viewport_width: f32 = aspect_ratio * viewport_height;
        // let focal_length: f32 = 1.0;

        return Camera {
            origin: lookfrom,
            lower_left_corner: llc,
            horizontal: h,
            vertical: v,
            u: _u,
            v: _v,
            lens_radius: aperture / 2.0
        };
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = rd.x() * self.u + rd.y() * self.v;
        return Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset);
    }
}