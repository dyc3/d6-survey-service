#[macro_use]
extern crate rocket;
#[macro_use]
extern crate typeshare;
#[macro_use]
extern crate diesel;

pub mod api;
mod db;
pub mod jwt;
mod questions;
mod survey;
mod user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::stage()).mount(
        "/api",
        routes![
            index,
            user::register_user,
            user::login_user,
            survey::create_survey,
            survey::get_survey,
            survey::get_survey_auth,
            survey::edit_survey
        ],
    )
}
