mod vec3;
mod ray;
mod hittable;
mod hittable_list;

use vec3::Vec3;
use vec3::Color;
use vec3::Point3;
use ray::Ray;

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 400;
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical/2.0) - Vec3::new(0.0, 0.0, focal_length);


    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        if j % 10 == 0 {
            eprintln!("\nScanlines remaining: {} ", j);
        }
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(r);
            write_color(&pixel_color);
        }
    }
    eprintln!("\nCompleted!\n");


    // image_output(IMAGE_HEIGHT, IMAGE_WIDTH);
    // test_vec3();
}

fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(&(Point3::new(0.0, 0.0, -1.0)), 0.5, &r);
    if t > 0.0 {
        let N: Vec3 = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction() as Vec3);
    t = 0.5 * (unit_direction.y() + 1.0);
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

fn image_output(IMAGE_WIDTH: i32, IMAGE_HEIGHT: i32) {
    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\nScanlines remaining: {} ", i);
        for j in 0..IMAGE_WIDTH {
            let r = (j - 1) as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = (i - 1) as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let v: Vec3 = Vec3::new(r, g, b);
            write_color(&v);
        }
    }
    eprintln!("\nCompleted!\n");
}

pub fn write_color(v: &Color) {
    println!("{} {} {}", (255.999 * v.x()) as i32, (255.999 * v.y()) as i32, (255.999 * v.z()) as i32);
}
