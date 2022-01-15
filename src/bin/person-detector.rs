extern crate person_detector;
extern crate image;

use person_detector::skin::rgb_check;

use image::{Rgb};

const INPUT_PATH: &str = "input.jpg";
const OUTPUT_PATH: &str = "output.png";

fn main() {

	const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
	const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

	// Opening image and converting in to rgb8
	let mut mut_img = image::open(INPUT_PATH).unwrap().into_rgb8();

	// Getting width and height
	let (width, height) = mut_img.dimensions();

	for x in 0..width {
		for y in 0..height {
			let pixel = mut_img.get_pixel(x, y);

			if rgb_check((pixel[0], pixel[1], pixel[2])) {
				mut_img.put_pixel(x, y, WHITE);
			} else {
				mut_img.put_pixel(x, y, BLACK);
			};
		}
	}
	mut_img.save(OUTPUT_PATH).unwrap();
}
