use person_detector::skin::rtslrgb_check;

use image::Rgb;

const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

const INPUT_PATH: &str = "input.jpg";
const OUTPUT_PATH: &str = "output.png";

fn main() {
	// Opening image and converting in to rgb8
	let mut mut_img = image::open(INPUT_PATH).unwrap().into_rgb8();

	// Getting width and height
	let (width, height) = mut_img.dimensions();

	for x in 0..width {
		for y in 0..height {
			let pixel = mut_img.get_pixel(x, y);

			let color = if rtslrgb_check(pixel[0], pixel[1], pixel[2]) {
				WHITE
			} else {
				BLACK
			};

			mut_img.put_pixel(x, y, color);
		}
	}

	mut_img.save(OUTPUT_PATH).unwrap();
}
