#[macro_use]
extern crate rocket;
#[macro_use]
extern crate typeshare;

mod questions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
