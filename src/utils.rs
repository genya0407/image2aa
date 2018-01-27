extern crate image;

use ndarray::*;
use image::{ImageDecoder, DecodingResult, ColorType};
use string_error::{static_err};
use std::error::Error;
use std::io::{Read, Write, Cursor};

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

pub fn read_image<R: Read>(mut image_file: R) -> Result<Array3<f32>, Box<Error>> {
    let mut image_buffer = Vec::new();
    image_file.read_to_end(&mut image_buffer)?;
    let format = image::guess_format(&image_buffer)?;
    let image_reader = Cursor::new(image_buffer);
    let abstract_image_data = match format {
        image::ImageFormat::PNG => {
            let mut decoder = image::png::PNGDecoder::new(image_reader);
            let result = decoder.read_image()?;
            let (x, y) = decoder.dimensions()?;
            let colortype = decoder.colortype()?;
            Ok((result, (x, y), colortype))
        },
        image::ImageFormat::JPEG => {
            let mut decoder = image::jpeg::JPEGDecoder::new(image_reader);
            let result = decoder.read_image()?;
            let (x, y) = decoder.dimensions()?;
            let colortype = decoder.colortype()?;
            Ok((result, (x, y), colortype))
        },
        _ => Err(static_err("Unsupported file type. Only PNG and JPEG are supported.")),
    };
    let (result, (x, y), colortype) = abstract_image_data?;
    
    let raw_data = match result {
        DecodingResult::U8(v) => v.into_iter().map(|e| e as f32).collect::<Vec<f32>>(),
        DecodingResult::U16(v) => v.into_iter().map(|e| e as f32).collect::<Vec<f32>>(),
    };
    let arr = Array1::<f32>::from(raw_data);

    match colortype {
        ColorType::RGBA(_) =>
            arr.into_shape((y as Ix, x as Ix, 4 as Ix)).map_err(|_| static_err("Wrong shape!")),
        ColorType::RGB(_) =>
            arr.into_shape((y as Ix, x as Ix, 3 as Ix)).map_err(|_| static_err("Wrong shape!")),
        _ => Err(static_err("Unsupported colortype")),
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