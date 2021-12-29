extern crate person_detector;
extern crate image;

use person_detector::skin::rgb_check;

use std::env;
use std::process::exit;

use image::Rgb;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input> <output>", args[0]);
        exit(1);
    }

    let img = image::open(&args[1]).unwrap();

    let black: Rgb<u8> = image::Rgb([0, 0, 0]);
    let white: Rgb<u8> = image::Rgb([255, 255, 255]);

	let width = img.width();
	let height = img.height();

	let mut mut_img = img.clone().into_rgb8();

	for x in 0..width {
		for y in 0..height {
            let pixel = img.get_pixel(x, y);

            let color = if rgb_check((pixel[0], pixel[1], pixel[2])) {
                white
            } else {
                black
            };

			mut_img.put_pixel(x, y, color);
		}
	}

    mut_img.save(&args[2]).unwrap();
}
