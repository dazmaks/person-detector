extern crate person_detector;
extern crate image;

use person_detector::skin::rgb_check;

use std::{env, process};

use image::{Rgb, GenericImageView};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input> <output>", args[0]);
        process::exit(1);
    }

    let img = image::open(&args[1]).unwrap();

    const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
    const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

    // Opening image and converting in to rgb8
	let mut mut_img = image::open(&args[1]).unwrap().into_rgb8();

    // Getting width and height
	let (width, height) = mut_img.dimensions();

	for x in 0..width {
		for y in 0..height {
            let pixel = img.get_pixel(x, y);

            let color = if rgb_check((pixel[0], pixel[1], pixel[2])) {
                WHITE
            } else {
                BLACK
            };

			mut_img.put_pixel(x, y, color);
		}
	}
    mut_img.save(&args[2]).unwrap();
}
