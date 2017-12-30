#![feature(slice_patterns)]
//#[macro_use(s)]

extern crate image;
#[macro_use] extern crate ndarray;
extern crate string_error;
extern crate getopts;

use image::{ImageDecoder, DecodingResult, ColorType};
use std::fs::File;
use ndarray::*;
use std::error::Error;
use string_error::{static_err};
use std::env;
use getopts::Options;

mod filter;

fn read_png(filename: String) -> Result<Array3<f32>, Box<Error>> {
    let image_file = File::open(filename)?;
    let mut decoder = image::png::PNGDecoder::new(image_file);
    let result = decoder.read_image()?;
    let (x, y) = decoder.dimensions()?;
    match result {
        DecodingResult::U8(v) => {
            let arr = Array1::<f32>::from(v.into_iter().map(|e| e as f32).collect::<Vec<f32>>());
            match decoder.colortype() {
                Ok(ColorType::RGBA(_)) => arr.into_shape((y as Ix, x as Ix, 4 as Ix)).map_err(|_| static_err("Wrong shape!")),
                Ok(ColorType::RGB(_)) => arr.into_shape((y as Ix, x as Ix, 3 as Ix)).map_err(|_| static_err("Wrong shape!")),
                _ => Err(static_err("Unsupported colortype")),
            }
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

fn setup_option_parser() -> Options {
    let mut opts = Options::new();
    opts.optopt("s", "blocksize", "set bocksize (default: 32)", "SIZE");
    opts.reqopt("i", "input", "input file path", "FILE");
    opts.optopt("", "char-detect-thresh", "threshould for character detection (default: 10)", "THRESH");
    opts.optopt("", "line-detect-thresh", "threshould for line detection (default: 10)", "THRESH");
    return opts;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parser = setup_option_parser();
    let matches = match parser.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    let input_file = matches.opt_str("i").unwrap();

    let mut hough_filter = filter::block_hough::default();
    if let Some(block_size_str) = matches.opt_str("s") {
        hough_filter.block_size = block_size_str.parse().unwrap();
    }
    if let Some(slope_count_thresh_str) = matches.opt_str("char-detect-thresh") {
        hough_filter.slope_count_thresh = slope_count_thresh_str.parse().unwrap();
    }

    let mut binary_filter = filter::binary::default();
    if let Some(line_detect_thresh_str) = matches.opt_str("line-detect-thresh") {
        binary_filter.thresh = line_detect_thresh_str.parse().unwrap();
    }

    let image_array = read_png(input_file).unwrap();
    let grayscale_array = filter::grayscale::default().run(image_array);
    let gradient_array = filter::line::default().run(grayscale_array.clone());
    let line_array = binary_filter.run(gradient_array);
    write_grayscale_png(String::from("out/line.png"), &line_array).unwrap();
    let hough_array = hough_filter.run(line_array);
    let aa = filter::ascii_art::default().run(hough_array);
    println!("{}", aa);
}
