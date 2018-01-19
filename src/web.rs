#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate ndarray;
extern crate rocket;
extern crate rocket_contrib;
extern crate image;
extern crate string_error;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io;
use std::io::{Read, Cursor};
use rocket_contrib::{Json, Value};
use rocket::response::NamedFile;

mod filter;
mod utils;

#[derive(FromForm)]
struct Options {
    blocksize: Option<usize>,
    char_detect_thresh: Option<u32>,
    line_detect_thresh: Option<u32>
}


#[derive(Serialize)]
struct Res {
    aa: String
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[post("/image", data = "<image_binary>")]
fn image_without_options(image_binary: rocket::Data) -> Json<Res> {
    let options = Options { blocksize: None, char_detect_thresh: None, line_detect_thresh: None };
    image(options, image_binary)
}

#[post("/image?<options>", data = "<image_binary>")]
fn image(options: Options, image_binary: rocket::Data) -> Json<Res> {
    let mut hough_filter = filter::hough::default();
    if let Some(block_size) = options.blocksize { hough_filter.block_size = block_size; }
    if let Some(slope_count_thresh) = options.char_detect_thresh { hough_filter.slope_count_thresh = slope_count_thresh; }

    let mut binary_filter = filter::binary::default();
    if let Some(thresh) = options.line_detect_thresh { binary_filter.thresh = thresh; }

    let image_array = utils::read_png(Box::new(image_binary.open())).map_err(|e| println!("{}", e)).unwrap();

    let grayscale_array = filter::grayscale::default().run(image_array);
    let gradient_array = filter::line::default().run(grayscale_array.clone());
    let line_array = binary_filter.run(gradient_array).mapv(|e| e as f32) * 250.;
    let hough_array = hough_filter.run(line_array);
    let aa = filter::ascii_art::default().run(hough_array);
    Json(Res { aa: aa })
}

fn main() {
    rocket::ignite().mount("/", routes![index, image, image_without_options]).launch();
}