use rocket::fairing::AdHoc;
use rocket::Build;
use rocket::Rocket;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

pub mod models;
pub mod schema;

#[database("survey_app")]
pub struct Storage(diesel::PgConnection);

embed_migrations!("migrations");

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = Storage::get_one(&rocket)
        .await
        .expect("database connection");

    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket
            .attach(Storage::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
