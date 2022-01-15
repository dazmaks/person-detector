use std::f64::consts::PI;

const PI2: f64 = 2. * PI;

pub fn ts_from_rgb((r, g, b): (u8, u8, u8)) -> (f64, f64) {
	let (rf, gf, bf) = (r as f64, g as f64, b as f64);
	let rgbsum = rf + gf + bf;

	if rgbsum == 0. {
		return (0., 0.);
	}

	let r1 = rf / rgbsum - 1. / 3.;
	let g1 = gf / rgbsum - 1. / 3.;
	let tcalc = f64::atan2(r1, g1) / PI2;
	let t = if g1 > 0. {
		tcalc + 1. / 4.
	} else if g1 < 0. {
		tcalc + 3. / 4.
	} else {
		0.
	};

	let s = (9. / 5. * ( r1 * r1 + g1 * g1 )).sqrt();
	(t, s)
}
