#[macro_use]
extern crate rocket;
#[macro_use]
extern crate typeshare;
#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;
pub mod jwt;
pub mod questions;
pub mod survey;
// #[cfg(any(test, bench))]
pub mod test_helpers;
pub mod user;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().attach(db::stage()).mount(
        "/api",
        routes![
            index,
            user::register_user,
            user::login_user,
            user::list_surveys,
            survey::create_survey,
            survey::get_survey,
            survey::get_survey_auth,
            survey::edit_survey
        ],
    )
}