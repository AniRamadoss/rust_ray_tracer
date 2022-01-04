mod vec3;
mod ray;
mod hittable;
mod rtweekend;
mod sphere;
mod hittable_list;
mod camera;
mod lambertian;
mod material;
mod metal;

use sphere::Sphere;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;
use ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use camera::Camera;
use lambertian::Lambertian;
use crate::metal::Metal;

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 400;
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const samples_per_pixel: i32 = 100;
    const max_depth: i32 = 50;

    // World
    let world_objects: Vec<Box<dyn Hittable>> = Vec::new();
    let mut world = HittableList::new(world_objects);
    let material_ground: Box<Lambertian> = Box::new(Lambertian {albedo: Color::new(0.8, 0.8, 0.0)});
    let material_center: Box<Lambertian> = Box::new(Lambertian {albedo: Color::new(0.7, 0.3, 0.3)});
    let material_left: Box<Metal> = Box::new(Metal {albedo: Color::new(0.8, 0.8, 0.8)});
    let material_right: Box<Metal> = Box::new(Metal {albedo: Color::new(0.8, 0.6, 0.2)});

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));




    // Camera
    let cam: Camera = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        if j % 10 == 0 {
            eprintln!("\nScanlines remaining: {} ", j);
        }
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f32 + rtweekend::random_double()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rtweekend::random_double()) / (IMAGE_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }

    }
    eprintln!("\nCompleted!\n");


    // image_output(IMAGE_HEIGHT, IMAGE_WIDTH);
    // test_vec3();
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if (*world).hit(&r, 0.001, rtweekend::infinity as f32, &mut rec) {
        let scattered: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation: Color = Color::new(0.0, 0.0, 0.0);
        if rec.mat_ptr.scatter(&r, &rec, Box::new(attenuation), Box::new(scattered)) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
    }

    let unit_direction: Vec3 = Vec3::unit_vector(r.direction() as Vec3);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

pub fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = (*r).origin() - *center;
    let a = (*r).direction().length_squared();
    let half_b = Vec3::dot(oc, (*r).direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - (discriminant).sqrt()) / (a)
    }
}

pub fn write_color(v: Color, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f32);
    let r: f32 = (v.x() * scale).sqrt();
    let g: f32 = (v.y() * scale).sqrt();
    let b: f32 = (v.z() * scale).sqrt();

    println!("{} {} {}", (256.0 * rtweekend::clamp(r, 0.0, 0.999)) as i32, (256.0 * rtweekend::clamp(g, 0.0, 0.999)) as i32, (256.0 * rtweekend::clamp(b, 0.0, 0.999)) as i32);
}

fn test_vec3() {
    // Testing vec3.rs
    let vec1 : Vec3 = Vec3::new(1f32, 2f32, 3f32);
    let vec2 : Vec3 = Vec3::new(1f32, 2f32, 3f32);
    let result = vec1 + vec2;
    assert_eq!(result.x(), 2.0);
    assert_eq!(result.y(), 4.0);
    assert_eq!(result.z(), 6.0);
    assert_eq!(vec1.x() + vec2.x(),result.x());
    assert_eq!(vec1.length(), (14 as f32).sqrt());
    let vec3: Vec3 = Vec3::new(7f32, 14f32, 21f32);
    assert_same_vector(&vec3, &(7.0 * vec1));
    assert_same_vector(&(vec3 / 7.0), &vec1);

    let vec1 : Vec3 = Vec3::new(3f32, 7f32, 31f32);
    let vec2 : Vec3 = Vec3::new(36f32, 64f32, 8f32);
    let cross_vec : Vec3 = Vec3::new(-1928f32, 1092f32, -60f32);
    assert_same_vector(&cross_vec, &Vec3::cross(vec1, vec2));

    assert_eq!(804 as f32, Vec3::dot(vec1, vec2));
}

fn assert_same_vector(v1: &Vec3, v2: &Vec3) {
    assert_eq!(v1.x(), v2.x());
    assert_eq!(v1.y(), v2.y());
    assert_eq!(v1.z(), v2.z());
}

// fn image_output(IMAGE_WIDTH: i32, IMAGE_HEIGHT: i32) {
//     // Render
//     print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
//     for i in (0..IMAGE_HEIGHT).rev() {
//         eprintln!("\nScanlines remaining: {} ", i);
//         for j in 0..IMAGE_WIDTH {
//             let r = (j - 1) as f32 / (IMAGE_WIDTH - 1) as f32;
//             let g = (i - 1) as f32 / (IMAGE_HEIGHT - 1) as f32;
//             let b = 0.25;
//
//             let v: Vec3 = Vec3::new(r, g, b);
//             write_color(&v);
//         }
//     }
//     eprintln!("\nCompleted!\n");
// }


