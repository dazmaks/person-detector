use std::f32::consts::PI;
use fast_math::atan;

const PI2: f32 = 2. * PI;
const ONETHIRD: f32 = 1. / 3.;

pub fn ts_from_rgb(r: u8, g: u8, b: u8) -> (f32, f32) {
	let (rf, gf, bf) = (r as f32, g as f32, b as f32);
	let rgbsum = rf + gf + bf;

	let r1 = rf / rgbsum - ONETHIRD;
	let g1 = gf / rgbsum - ONETHIRD;
	let tcalc = atan(r1 / g1) / PI2;
	let t = if g1 > 0. {
		tcalc + 1. / 4.
	} else if g1 < 0. {
		tcalc + 3. / 4.
	} else {
		0.
	};

	(t, (9. / 5. * ( r1 * r1 + g1 * g1 )).sqrt())
}
