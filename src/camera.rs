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
    pub fn new(vfov: f32, aspect_ratio: f32) -> Camera {
        const FOCAL_LENGTH: f32 = 1.0;

        // Vertical FOV in Deg
        let theta = degrees_to_radians(vfov as f64);
        let viewport_height = 2.0 * ((theta as f32) / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        return Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: Point3::new(0.0, 0.0, 0.0) - ((Vec3::new(viewport_width, 0.0, 0.0)) / 2.0) - (Vec3::new(0.0, viewport_height, 0.0) / 2.0) - Vec3::new(0.0, 0.0, focal_length),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin);
    }
}