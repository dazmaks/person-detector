use std::f64::consts::PI;

const PI2: f64 = 2. * PI;

pub fn ts_from_rgb(r: u8, g: u8, b: u8) -> (f64, f64) {
	let (rf, gf, bf) = (r as f64, g as f64, b as f64);
	let rgbsum = rf + gf + bf;

	let r1 = rf / rgbsum - 1. / 3.;
	let g1 = gf / rgbsum - 1. / 3.;
	let tcalc = (r1 / g1).atan() / PI2;
	let t = if g1 > 0. {
		tcalc + 1. / 4.
	} else if g1 < 0. {
		tcalc + 3. / 4.
	} else {
		0.
	};

	(t, (9. / 5. * ( r1 * r1 + g1 * g1 )).sqrt())
}
