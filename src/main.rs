fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n {0} {1} \n255\n", image_width, image_height);

    for i in (0..image_height - 1).rev() {
        println!("\rScanlines remaining: {}", i);
        for j in 0..image_width {
            let r = (j as f32) / (image_width as f32 - 1 as f32);
            let g = (i as f32) / (image_height as f32 - 1 as f32);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            print!("{0} {1} {2} \n", ir, ig, ib)
        }
    }
}
