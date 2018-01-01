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
use rocket::response::NamedFile;
use std::collections::HashMap;
use ndarray::Array3;
use std::sync::RwLock;

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

#[get("/image/<image_uuid>")]
fn image_without_options(image_uuid: String, image_store: rocket::State<RwLock<ImageStore>>) -> Json<Res> {
    let options = Options { blocksize: None, char_detect_thresh: None, line_detect_thresh: None };
    image(image_uuid, image_store, options)
}

#[get("/image/<image_uuid>?<options>")]
fn image(image_uuid: String, image_store: rocket::State<RwLock<ImageStore>>, options: Options) -> Json<Res> {
    let mut hough_filter = filter::block_hough::default();
    if let Some(block_size) = options.blocksize { hough_filter.block_size = block_size; }
    if let Some(slope_count_thresh) = options.char_detect_thresh { hough_filter.slope_count_thresh = slope_count_thresh; }

    let mut binary_filter = filter::binary::default();
    if let Some(thresh) = options.line_detect_thresh { binary_filter.thresh = thresh; }

    if let Some(image_array) = image_store.read().unwrap().get(image_uuid) {
        let grayscale_array = filter::grayscale::default().run(image_array);
        let gradient_array = filter::line::default().run(grayscale_array.clone());
        let line_array = binary_filter.run(gradient_array).mapv(|e| e as f32) * 250.;
        let hough_array = hough_filter.run(line_array);
        let aa = filter::ascii_art::default().run(hough_array);
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
        .mount("/", routes![index, image, image_without_options, load_image])
        .launch();
}