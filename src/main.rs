extern crate image;

use image::{ImageBuffer, Rgba, RgbaImage};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path_1 = &args[1];
    let image_path_2 = &args[2];
    let image_output_path = &args[3];
    println!("Image 1: {}", image_path_1);
    println!("Image 2: {}", image_path_2);

    // Images to diff
    let dynamic_image1 = image::open(image_path_1).unwrap();
    let image1 = dynamic_image1.as_rgba8().unwrap();
    let dynamic_image2 = image::open(image_path_2).unwrap();
    let image2 = dynamic_image2.as_rgba8().unwrap();

    let width = image1.width();
    let height = image1.height();

    let mut output_image: RgbaImage = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    for x in (0..width) {
        for y in (0..height) {
            let image1_pixel = image1.get_pixel(x, y).0;
            let image2_pixel = image2.get_pixel(x, y).0;
            if image1_pixel != image2_pixel {
                output_image.get_pixel_mut(x, y).0 = image2_pixel;
            }
        }
    }

    // write it out to a file
    output_image.save(image_output_path).unwrap();
}
