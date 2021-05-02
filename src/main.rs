#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello Rocket"
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, name]).launch();
}