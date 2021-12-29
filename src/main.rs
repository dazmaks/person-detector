extern crate image;

use std::env;
use std::process::exit;

use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input> <output>", args[0]);
        exit(1);
    }

    let img = image::open(&args[1]).unwrap();

    println!("dimensions {:?}", img.dimensions());

    println!("{:?}", img.color());

    img.save(&args[2]).unwrap();
}
