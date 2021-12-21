mod vec3;
use vec3::Vec3;

fn main() {
    // // Image dimensions
    // const IMAGE_WIDTH: i32 = 256;
    // const IMAGE_HEIGHT: i32 = 256;
    // image_output(IMAGE_HEIGHT, IMAGE_WIDTH);

    // Testing vec3.rs
    let vec1 : Vec3 = Vec3::new(1f32, 2f32, 3f32);
    let vec2 : Vec3 = Vec3::new(1f32, 2f32, 3f32);
    let result = vec1 + vec2;
    assert_eq!(result.x(), 2.0);
    assert_eq!(result.y(), 4.0);
    assert_eq!(result.z(), 6.0);
    assert_eq!(vec1.x() + vec2.x(),result.x());
    assert_eq!(vec1.length(), (14 as f32).sqrt());
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