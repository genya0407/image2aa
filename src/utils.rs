extern crate image;

use ndarray::*;
use image::{ImageDecoder, DecodingResult, ColorType};
use string_error::{static_err};
use std::error::Error;
use std::io::{Read, Write};

pub fn convolve2d(base_arr: &Array2<f32>, filter: &Array2<f32>) -> Array2<f32> {
    let ys: isize = base_arr.shape()[0] as isize;
    let xs: isize = base_arr.shape()[1] as isize;
    let mut result: Array2<f32> = Array2::<f32>::zeros((ys as usize, xs as usize));
    for yt in 1..((result.shape()[0]-1) as isize) {
        for xt in 1..((result.shape()[1]-1) as isize) {
            result[[yt as usize, xt as usize]] = (&base_arr.slice(s![yt-1..yt+2, xt-1..xt+2]) * filter).scalar_sum();
        }
    }
    return result;
}

pub fn read_png(image_file: Box<Read>) -> Result<Array3<f32>, Box<Error>> {
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

pub fn write_grayscale_png(image_file: Box<Write>, img: &Array2<f32>) -> Result<(), Box<Error>> {
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