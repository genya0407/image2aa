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
extern crate uuid;

use uuid::Uuid;
use std::io;
use rocket_contrib::Json;
use rocket::response::{NamedFile, Stream};
use std::collections::HashMap;
use ndarray::Array3;
use std::sync::RwLock;
use std::fs::File;
use std::io::Read;

mod filter;
mod utils;

fn hough_filter_with_options(options: &Options) -> filter::block_hough::BlockHoughFilter {
    let mut hough_filter = filter::block_hough::default();
    if let Some(block_size) = options.blocksize { hough_filter.block_size = block_size; }
    if let Some(slope_count_thresh) = options.char_detect_thresh { hough_filter.slope_count_thresh = slope_count_thresh; }
    return hough_filter;    
}

fn binary_filter_with_options(options: &Options) -> filter::binary::BinaryFilter {
    let mut binary_filter = filter::binary::default();
    if let Some(thresh) = options.line_detect_thresh { binary_filter.thresh = thresh; }
    return binary_filter;
}

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

#[post("/load_image", data = "<image_binary>")]
fn load_image(image_binary: rocket::Data, image_store: rocket::State<RwLock<ImageStore>>) -> String {
    {
        let im_store_read = image_store.read().unwrap();
        for key in im_store_read.image_hashmap.keys() {
            println!("{}", key);
        }
    }
    let uuid = Uuid::new_v4().simple().to_string();
    let image_array = utils::read_png(Box::new(image_binary.open())).map_err(|e| println!("{}", e)).unwrap();
    let mut im_store = image_store.write().unwrap();
    im_store.image_hashmap.insert(uuid.clone(), image_array);
    uuid
}

fn grayscale_image(
        image_uuid: String, options: Options,
        image_store: rocket::State<RwLock<ImageStore>>
    ) -> io::Result<Stream<Read>> {
    let binary_filter = binary_filter_with_options(&options);
    let grayscale_filter = filter::grayscale::default();
    let gradient_filter = filter::line::default();

    if let Some(image_array) = image_store.read().unwrap().get(image_uuid) {
        let grayscale_array = grayscale_filter.run(image_array);
        let gradient_array = gradient_filter.run(grayscale_array);
        let line_array = binary_filter.run(gradient_array).mapv(|e| e as f32) * 250.;
        let mut buff = io::Cursor::new(Vec::<u8>::new());
        utils::write_grayscale_png(Box::new(buff), &line_array);

        Ok(Stream::from(buff))
    } else {
        panic!("no such image!")
    }

}

#[get("/aa/<image_uuid>")]
fn aa_without_options(image_uuid: String, image_store: rocket::State<RwLock<ImageStore>>) -> Json<Res> {
    let options = Options { blocksize: None, char_detect_thresh: None, line_detect_thresh: None };
    aa(image_uuid, image_store, options)
}

#[get("/aa/<image_uuid>?<options>")]
fn aa(image_uuid: String, image_store: rocket::State<RwLock<ImageStore>>, options: Options) -> Json<Res> {
    let hough_filter = hough_filter_with_options(&options);
    let binary_filter = binary_filter_with_options(&options);
    let grayscale_filter = filter::grayscale::default();
    let gradient_filter = filter::line::default();
    let ascii_art_filter = filter::ascii_art::default();

    if let Some(image_array) = image_store.read().unwrap().get(image_uuid) {
        let grayscale_array = grayscale_filter.run(image_array);
        let gradient_array = gradient_filter.run(grayscale_array.clone());
        let line_array = binary_filter.run(gradient_array).mapv(|e| e as f32) * 250.;
        let hough_array = hough_filter.run(line_array);
        let aa = ascii_art_filter.run(hough_array);
        Json(Res { aa: aa })
    } else {
        Json(Res { aa: String::from("") })
    }
}

#[derive(Debug)]
struct ImageStore {
    pub image_hashmap: HashMap<String, Array3<f32>>
}

impl ImageStore {
    pub fn get(&self, uuid: String) -> Option<Array3<f32>> {
        self.image_hashmap.get(&uuid).map(|arr| arr.clone())
    }
}

fn main() {
    rocket::ignite()
        .manage(RwLock::new(ImageStore { image_hashmap: HashMap::new() }))
        .mount("/", routes![index, aa, aa_without_options, load_image])
        .launch();
}