#[macro_use]
extern crate rocket;
#[macro_use]
extern crate typeshare;

mod db;
mod questions;
mod user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, user::register_user])
}
