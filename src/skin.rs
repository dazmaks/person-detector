use crate::tsl::ts_from_rgb;

#[inline(always)]
fn min(a: u8,b: u8) -> u8 {
	if a < b { a } else { b }
}

#[inline(always)]
fn max(a: u8,b: u8) -> u8 {
	if a > b { a } else { b }
}

#[inline(always)]
fn min3(a: u8, b: u8, c: u8) -> u8 {
	min(min(a, b), c)
}

#[inline(always)]
fn max3(a: u8, b: u8, c: u8) -> u8 {
	max(max(a, b), c)
}

// Basic rgb check
pub fn rgb_check((r, g, b): (u8, u8, u8)) -> bool {
	r > 90 &&
	g > 40 &&
	r > g &&
	r > b &&
	i8::abs(r as i8 - g as i8) > 15 &&
	max3(r, g, b) - min3(r, g, b) > 15
}

// Basic tsl + rgb check
pub fn tlsrgb_check((r, g, b): (u8, u8, u8)) -> bool {
	let (t, s) = ts_from_rgb((r, g, b));
	rgb_check((r, g, b)) && ( t - 0.578 ).abs() < 0.059 && (s - 0.157).abs() < 0.118
}
