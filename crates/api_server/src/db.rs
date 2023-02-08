use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("survey_app")]
struct Storage(sqlx::SqlitePool);
