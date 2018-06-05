// Copyright (C) 2017 - Will Glozer. All rights reserved.

extern crate image;
extern crate quirc;

use std::env::args;
use std::fs::File;
use std::io::Read;
use quirc::QrCoder;

fn main() {
    let arg = args().nth(1).expect("a file argument");

    let image = image::open(arg).unwrap().to_luma();

    let mut quirc = QrCoder::new().unwrap();

    let width = image.width();
    let height = image.height();
    let codes = quirc.codes(&image, width, height).unwrap();

    for code in codes {
        match code {
            Ok(code) => println!("{:?}", code),
            Err(err) => println!("{:?}", err),
        }
    }
}
