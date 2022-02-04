use crate::tsl::ts_from_rgb;

fn min(a: u8,b: u8) -> u8 {
	if a < b { a } else { b }
}

fn max(a: u8,b: u8) -> u8 {
	if a > b { a } else { b }
}

fn min3(a: u8, b: u8, c: u8) -> u8 {
	min(min(a, b), c)
}

fn max3(a: u8, b: u8, c: u8) -> u8 {
	max(max(a, b), c)
}

// Basic rgb check
pub fn rgb_check(r: u8, g: u8, b: u8) -> bool {
	r > 90 && g > 40 && r > g && r > b && r - g > 15 &&
	max3(r, g, b) - min3(r, g, b) > 15
}

// Basic tsl + rgb check
pub fn tslrgb_check(r: u8, g: u8, b: u8) -> bool {
	let (t, s) = ts_from_rgb(r, g, b);
	rgb_check(r, g, b) && (t-0.578).abs() < 0.059 && (s-0.157).abs() < 0.118
}

// Rewrite of tsl + rgb check
pub fn rtslrgb_check(r: u8, g: u8, b: u8) -> bool {
	let (t, s) = ts_from_rgb(r, g, b);
	r > 210 && r < 255 && g > 188 && g < 248 && b > 135 && b < 220 && (t-0.578).abs() < 0.059 && (s-0.157).abs() < 0.118
}
