#![feature(slice_patterns)]
//#[macro_use(s)]

extern crate image;
#[macro_use] extern crate ndarray;
extern crate string_error;

use image::{ImageDecoder, DecodingResult, ColorType};
use std::fs::File;
use ndarray::*;
use std::error::Error;
use string_error::{static_err};
use std::env;

mod filter;

fn read_png(filename: String) -> Result<Array3<f32>, Box<Error>> {
    let image_file = File::open(filename)?;
    let mut decoder = image::png::PNGDecoder::new(image_file);
    let result = decoder.read_image()?;
    let (x, y) = decoder.dimensions()?;
    match result {
        DecodingResult::U8(v) => {
            let arr = Array1::<f32>::from(v.into_iter().map(|e| e as f32).collect::<Vec<f32>>());
            arr.into_shape((y as Ix, x as Ix, 4 as Ix)).map_err(|_| static_err("Wrong shape!"))
        }
        DecodingResult::U16(_) => Err(static_err("Unsupported bit depth!"))
    }
}

fn write_grayscale_png(filename: String, img: &Array2<f32>) -> Result<(), Box<Error>> {
    let image_file = File::create(filename)?;
    let decoder = image::png::PNGEncoder::new(image_file);
    let shape = img.shape();
    let height = shape[0] as u32;
    let width = shape[1] as u32;
    let u8img = img.clone().map(|e| *e as u8);
    let data = u8img.as_slice().unwrap();
    decoder.encode(
        data,
        width, height, ColorType::Gray(8)
    ).map_err(|_| static_err("Error writing image!"))
}

fn main() {
    let input_file = env::args().nth(1).unwrap();
    let image_array = read_png(input_file).unwrap();
    let grayscale_array = filter::grayscale::default().run(image_array);
    let gradient_array = filter::line::default().run(grayscale_array.clone());
    let line_array = filter::binary::default().run(gradient_array);
    write_grayscale_png(String::from("out/line.png"), &line_array).unwrap();
    let hough_filter = filter::block_hough::default();
    let hough_array = hough_filter.run(line_array);
    let aa = filter::ascii_art::default().run(hough_array);
    println!("{}", aa);
}
