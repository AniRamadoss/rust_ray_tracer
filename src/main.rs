fn main() {
    // Image dimensions
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for i in (0..IMAGE_HEIGHT).rev() {
        for j in 0..IMAGE_WIDTH {
            let r = (j - 1) as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = (i - 1) as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let r_value = (255.999 * r) as i32;
            let g_value = (255.999 * g) as i32;
            let b_value = (255.999 * b) as i32;
            print!("{} {} {}\n", r_value, g_value, b_value);
        }
    }
}