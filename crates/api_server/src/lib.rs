#[macro_use]
extern crate rocket;
#[macro_use]
extern crate typeshare;
#[macro_use]
extern crate diesel;

pub mod api;
mod cache;
mod cors;
pub mod db;
pub mod jwt;
pub mod questions;
pub mod survey;
pub mod survey_response;
// #[cfg(any(test, bench))]
pub mod test_helpers;
pub mod user;
pub mod validate;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .attach(cors::Cors)
        .mount("/", routes![cors::handle_preflight])
        .mount(
            "/api",
            routes![
                api::health,
                user::register_user,
                user::login_user,
                user::list_surveys,
                survey::create_survey,
                survey::get_survey,
                survey::get_survey_auth,
                survey::edit_survey,
                survey::delete_survey,
                survey_response::create_survey_response,
                survey_response::edit_survey_response,
                survey_response::get_survey_response,
            ],
        )
        .register("/", catchers![api::default_catcher])
}
