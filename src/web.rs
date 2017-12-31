#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate ndarray;
extern crate rocket;
extern crate image;
extern crate string_error;

mod filter;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}