extern crate image;

use image::{ImageBuffer, Rgba, RgbaImage};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path_1 = &args[1];
    let image_path_2 = &args[2];
    let image_output_path = &args[3];
    let mut leniency = 0.0;
    if args.len() > 4 {
        leniency = args[4].parse().unwrap_or(0.0);
    }
    println!("Image 1: {}", image_path_1);
    println!("Image 2: {}", image_path_2);

    // Images to diff
    let dynamic_image1 = image::open(image_path_1).unwrap();
    let image1 = dynamic_image1.to_rgba();
    let dynamic_image2 = image::open(image_path_2).unwrap();
    let image2 = dynamic_image2.to_rgba();

    let width = image1.width();
    let height = image1.height();

    let mut output_image: RgbaImage = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let image1_pixel = image1.get_pixel(x, y).0;
            let image2_pixel = image2.get_pixel(x, y).0;
            let r1 = image1_pixel[0] as i32;
            let g1 = image1_pixel[1] as i32;
            let b1 = image1_pixel[2] as i32;
            let a1 = image1_pixel[3] as i32;
            let r2 = image2_pixel[0] as i32;
            let g2 = image2_pixel[1] as i32;
            let b2 = image2_pixel[2] as i32;
            let a2 = image2_pixel[3] as i32;
            if pixels_not_equal(image1_pixel, image2_pixel, leniency) {
                output_image.get_pixel_mut(x, y).0 = image2_pixel;
            }
        }
    }

    // write it out to a file
    output_image.save(image_output_path).unwrap();
}

fn pixels_not_equal(pixel1: [u8; 4], pixel2: [u8; 4], leniency: f64) -> bool {
    let difference: f64 = (((pixel1[0] as f64 - pixel2[0] as f64).abs() / 255.0)
        + ((pixel1[1] as f64 - pixel2[1] as f64).abs() / 255.0)
        + ((pixel1[2] as f64 - pixel2[2] as f64).abs() / 255.0)
        + ((pixel1[3] as f64 - pixel2[3] as f64).abs() / 255.0))
        / 4.0;
    return difference > leniency;
}
