extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::Font;
use rusttype::Scale;

pub fn text2image(text: String) -> image::RgbImage {
    let font = Vec::from(include_bytes!("../assets/font/VL-Gothic-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let x_font_size = 5f32;
    let y_font_size = 10f32;
    let scale = Scale {
        x: x_font_size * 2.,
        y: y_font_size,
    };

    let longest_line_length = text.lines().map(font_ascii_count).max().unwrap() / 2;
    let line_count = text.lines().count();

    let image_width = ((x_font_size * 2.) * longest_line_length as f32) as u32;
    let image_height = (line_count as f32 * y_font_size) as u32;
    let mut image = RgbImage::from_pixel(image_width, image_height, Rgb([255, 255, 255]));

    for (v_index, line) in text.lines().enumerate() {
        let mut h_index = 0u32;
        for c in line.chars() {
            draw_text_mut(
                &mut image,
                Rgb([0u8, 0u8, 0u8]),
                (h_index as f32 * x_font_size) as u32,
                (v_index as f32 * y_font_size) as u32,
                scale,
                &font,
                &c.to_string(),
            );

            if c.is_ascii() {
                h_index += 1;
            } else {
                h_index += 2;
            }
        }
    }

    return image;
}

fn font_ascii_count(text: &str) -> u32 {
    text.chars()
        .map(|char| if char.is_ascii() { 1 } else { 2 })
        .sum::<u32>()
}
