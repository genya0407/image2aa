extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate image;
extern crate image2aa;
extern crate image2aa_web;
extern crate rand;
extern crate sha3;
extern crate time;

use image2aa::{filter, utils};
use rand::Rng;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::{json::Json, Serialize};
use rocket::{Request, Response};
use std::path::Path;

struct ContentDisposition(NamedFile, String);

impl<'r, 'o: 'r> Responder<'r, 'o> for ContentDisposition {
    fn respond_to(self, request: &'r Request) -> Result<Response<'o>, Status> {
        let filename = self.1.clone();
        match self.0.respond_to(request) {
            Ok(mut response) => {
                response.adjoin_raw_header(
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", filename),
                );
                Ok(response)
            }
            Err(status) => Err(status),
        }
    }
}

#[derive(FromForm)]
struct Options {
    blocksize: Option<usize>,
    char_detect_thresh: Option<u32>,
    line_detect_thresh: Option<u32>,
}

#[derive(FromForm)]
struct AsciiArtForm {
    text: String,
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[post("/download_aa_image", data = "<ascii_art>")]
async fn download_aa_image(ascii_art: Form<AsciiArtForm>) -> Option<ContentDisposition> {
    use std::time::SystemTime;

    let filename = format!(
        "/tmp/{}_{}.png",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        rand::thread_rng()
            .gen_ascii_chars()
            .take(20)
            .collect::<String>()
    );
    let path = Path::new(&filename);
    let image = image2aa_web::text2image(ascii_art.text.clone());
    image.save(path.clone()).unwrap();
    NamedFile::open(&filename)
        .await
        .map(|named_file| {
            ContentDisposition(
                named_file,
                path.file_name().unwrap().to_string_lossy().to_string(),
            )
        })
        .ok()
}

#[post("/image", data = "<image_binary>")]
async fn image_without_options<'r>(image_binary: rocket::Data<'r>) -> Json<AsciiArt> {
    let options = Options {
        blocksize: None,
        char_detect_thresh: None,
        line_detect_thresh: None,
    };
    image_with_option(options, image_binary).await
}

#[derive(Serialize)]
struct AsciiArt {
    aa: String,
}

#[post("/image?<options>", data = "<image_binary>")]
async fn image_with_option<'r>(options: Options, image_binary: rocket::Data<'r>) -> Json<AsciiArt> {
    use crate::rocket::tokio::io::AsyncReadExt;
    use rocket::data::ByteUnit;

    let mut image_buf = vec![];
    image_binary
        .open(30 * ByteUnit::MB)
        .read_to_end(&mut image_buf)
        .await
        .unwrap();

    let mut hough_filter = filter::hough::default();
    if let Some(block_size) = options.blocksize {
        hough_filter.block_size = block_size;
    }
    if let Some(slope_count_thresh) = options.char_detect_thresh {
        hough_filter.slope_count_thresh = slope_count_thresh;
    }

    let mut binary_filter = filter::binary::default();
    if let Some(thresh) = options.line_detect_thresh {
        binary_filter.thresh = thresh;
    }

    let image_array = utils::read_image(image_buf.as_slice())
        .map_err(|e| println!("{}", e))
        .unwrap();

    let grayscale_array = filter::grayscale::default().run(image_array);
    let gradient_array = filter::line::default().run(grayscale_array.clone());
    let line_array = binary_filter.run(gradient_array).mapv(|e| e as f32) * 250.;
    let hough_array = hough_filter.run(line_array);
    let aa = filter::ascii_art::default().run(hough_array);
    Json(AsciiArt { aa: aa })
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv::dotenv().ok();

    rocket::build().mount(
        "/",
        routes![
            index,
            image_with_option,
            image_without_options,
            download_aa_image
        ],
    )
}
