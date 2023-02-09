use std::time::Duration;

use diesel::r2d2::R2D2Connection;
use rocket::fairing::AdHoc;
use rocket::Build;
use rocket::Rocket;
use rocket_sync_db_pools::Config;
use rocket_sync_db_pools::PoolResult;
use rocket_sync_db_pools::Poolable;
use rocket_sync_db_pools::database;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod models;
pub mod schema;

#[database("survey_app")]
pub struct Storage(rocket_sync_db_pools::diesel::PgConnection);

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    Storage::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations");
        })
        .await;

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        rocket
            .attach(Storage::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
