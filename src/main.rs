use rayon::prelude::*;
use std;
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
mod dieletric;

use std::sync::Arc;
use rand::Rng;
use sphere::Sphere;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;
use ray::Ray;
use crate::hittable::Hittable;
use hittable_list::HittableList;
use camera::Camera;
use lambertian::Lambertian;
use crate::dieletric::Dielectric;
use crate::metal::Metal;


fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 1200;
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // Zoom
    // let R = f32::cos(PI/4.0);
    // let world_objects: Vec<Box<dyn Hittable>> = Vec::new();
    // let mut world = HittableList::new(world_objects);
    //
    // let mat_left = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    // let mat_right = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    //
    // let sphere_left = Sphere::new(Point3::new(-R, 0.0, -1.0), R, mat_left);
    // let sphere_right = Sphere::new(Point3::new(R, 0.0, -1.0), R, mat_right);
    // world.add(Box::new(sphere_left));
    // world.add(Box::new(sphere_right));


    // Defocus Blur

    // let world_objects: Vec<Box<dyn Hittable>> = Vec::new();
    // let mut world = HittableList::new(world_objects);
    // let material_ground: Arc<Lambertian> = Arc::new(Lambertian {albedo: Color::new(0.8, 0.8, 0.0)});
    // let material_center: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    // let mat_left_inner = Arc::new(Dielectric::new(1.5));
    // let material_right: Arc<Metal> = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    //
    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    // world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, mat_left_inner)));

    // Final Render
    let world = random_scene();



    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam: Camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);

        let scanline: Vec<Color> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let mut rng = rand::thread_rng();
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u as f32, v as f32);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }

            pixel_color
        }).collect();

        for pixel_color in scanline {
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nCompleted!\n");


    // image_output(IMAGE_HEIGHT, IMAGE_WIDTH);
    // test_vec3();
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let world_list: Vec<Box<dyn Hittable>> = Vec::new();
    let mut world = HittableList::new(world_list);

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sphere_ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    world.add(Box::new(sphere_ground));

    for i in -11..=11 {
        for j in -11..=11 {
            let choose_material = rng.gen::<f32>();
            let center = Point3::new((i as f32) + rng.gen_range(0.0..0.9), 0.2, (j as f32) + rng.gen_range(0.0..0.9));
            if choose_material < 0.8 {
                // Diffuse
                let albedo = Color::random_range(0.0, 1.0) * Color::random_range(0.0, 1.0);
                let sphere_material = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_material);

                world.add(Box::new(sphere));
            }

            else if choose_material < 0.95 {
                // Metal
                let albedo = Color::random_range(0.4, 1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_material);
                world.add(Box::new(sphere));
            }

            else {
                // Glass
                let sphere_material = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_material);
                world.add(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));
    return world;
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY as f32) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
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

// fn test_vec3() {
//     // Testing vec3.rs
//     let vec1 : Vec3 = Vec3::new(1f32, 2f32, 3f32);
//     let vec2 : Vec3 = Vec3::new(1f32, 2f32, 3f32);
//     let result = vec1 + vec2;
//     assert_eq!(result.x(), 2.0);
//     assert_eq!(result.y(), 4.0);
//     assert_eq!(result.z(), 6.0);
//     assert_eq!(vec1.x() + vec2.x(),result.x());
//     assert_eq!(vec1.length(), (14 as f32).sqrt());
//     let vec3: Vec3 = Vec3::new(7f32, 14f32, 21f32);
//     assert_same_vector(&vec3, &(7.0 * vec1));
//     assert_same_vector(&(vec3 / 7.0), &vec1);
//
//     let vec1 : Vec3 = Vec3::new(3f32, 7f32, 31f32);
//     let vec2 : Vec3 = Vec3::new(36f32, 64f32, 8f32);
//     let cross_vec : Vec3 = Vec3::new(-1928f32, 1092f32, -60f32);
//     assert_same_vector(&cross_vec, &Vec3::cross(vec1, vec2));
//
//     assert_eq!(804 as f32, Vec3::dot(vec1, vec2));
// }
//
// fn assert_same_vector(v1: &Vec3, v2: &Vec3) {
//     assert_eq!(v1.x(), v2.x());
//     assert_eq!(v1.y(), v2.y());
//     assert_eq!(v1.z(), v2.z());
// }

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


