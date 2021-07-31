extern crate getopts;
extern crate image2aa;

use getopts::Options;
use image2aa::{filter, utils};
use std::env;
use std::fs::File;

fn setup_option_parser() -> Options {
    let mut opts = Options::new();
    opts.optopt("s", "blocksize", "set bocksize (default: 32)", "SIZE");
    opts.optopt("i", "input", "input file path", "FILE");
    opts.optopt(
        "",
        "char-detect-thresh",
        "threshould for character detection (default: 10)",
        "THRESH",
    );
    opts.optopt(
        "",
        "line-detect-thresh",
        "threshould for line detection (default: 10)",
        "THRESH",
    );
    opts.optopt(
        "",
        "shrink-thresh",
        "threshould for shrink (default: 5)",
        "THRESH",
    );
    opts.optflag("", "help", "");
    return opts;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parser = setup_option_parser();
    let matches = match parser.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if !matches.opt_present("i") || matches.opt_present("help") {
        println!("{}", parser.short_usage("png2aa"));
        return;
    }

    let input_file = matches.opt_str("i").unwrap();

    let mut hough_filter = filter::hough::default();
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

    let mut shrink_filter = filter::shrink::default();
    if let Some(shrink_thresh_str) = matches.opt_str("shrink-thresh") {
        shrink_filter.thresh = shrink_thresh_str.parse().unwrap();
    }

    let image_array = utils::read_image(File::open(input_file).unwrap()).unwrap();
    let grayscale_array = filter::grayscale::default().run(image_array);
    let hough_array = hough_filter.run(grayscale_array);
    let aa = filter::ascii_art::default().run(hough_array);
    println!("{}", aa);
}
