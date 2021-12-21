mod vec3;
use vec3::Vec3;

fn main() {
    // // Image dimensions
    // const IMAGE_WIDTH: i32 = 256;
    // const IMAGE_HEIGHT: i32 = 256;
    // image_output(IMAGE_HEIGHT, IMAGE_WIDTH);
    test_vec3();
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
    assert_same_vector(&(7.0 / vec3), &vec1);

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

fn image_output(image_width: i32, image_height: i32) {
    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for i in (0..image_height).rev() {
        eprintln!("\nScanlines remaining: {} ", i);
        for j in 0..image_width {
            let r = (j - 1) as f32 / (image_width - 1) as f32;
            let g = (i - 1) as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let r_value = (255.999 * r) as i32;
            let g_value = (255.999 * g) as i32;
            let b_value = (255.999 * b) as i32;
            print!("{} {} {}\n", r_value, g_value, b_value);
        }
    }
    eprintln!("\nCompleted!\n");
}