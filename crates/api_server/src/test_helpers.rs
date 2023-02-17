use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
use jsonwebtoken::EncodingKey;
use rocket::local::blocking::Client;

use crate::{
    db::models::{Survey, SurveyPatch},
    jwt::Claims,
};

pub fn create_db_for_tests() -> String {
    let db_name = format!("survey_app_test_{}", uuid::Uuid::new_v4()).replace("-", "_");
    let mut conn = PgConnection::establish("postgres://vscode:notsecure@db/survey_app")
        .expect("Failed to connect to database");
    sql_query(format!("CREATE DATABASE {db_name}"))
        .execute(&mut conn)
        .expect("Failed to create test database");
    db_name.to_string()
}

pub fn drop_test_db(db_name: String) {
    let mut conn = PgConnection::establish("postgres://vscode:notsecure@db/survey_app")
        .expect("Failed to connect to database");
    sql_query(format!(
        "SELECT pg_terminate_backend(pg_stat_activity.pid FROM pg_stat_activity
            WHERE pg_stat_activity.datname = '{db_name}'
            AND pid <> pg_backend_pid();"
    ))
    .execute(&mut conn)
    .expect("Failed to kill connections to test database");
    sql_query(format!("DROP DATABASE IF EXISTS {db_name}"))
        .execute(&mut conn)
        .expect("Failed to drop test database");
}

pub fn create_test_user(client: &Client) -> String {
    let username = format!("test_user_{}", uuid::Uuid::new_v4());
    let response = client
        .post(uri!("/api", crate::user::register_user))
        .header(rocket::http::ContentType::JSON)
        .body(format!(
            r#"{{"username": "{username}", "password": "test"}}"#
        ))
        .dispatch();
    format!(
        "Bearer {}",
        response
            .into_json::<crate::user::UserToken>()
            .unwrap()
            .token
    )
}

pub fn make_jwt(client: &Client, user_id: i32) -> String {
    let key = EncodingKey::from_secret(client.rocket().config().secret_key.to_string().as_bytes());
    let claims = Claims::new(user_id);
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();
    "Bearer ".to_string() + &token
}

pub fn run_test_with_db<T>(test: T) -> ()
where
    T: FnOnce(&String) -> () + std::panic::UnwindSafe,
{
    let db_name = create_db_for_tests();
    let result = std::panic::catch_unwind(|| test(&db_name));
    drop_test_db(db_name);
    assert!(result.is_ok())
}

pub fn test_rocket(db_name: &String) -> rocket::Rocket<rocket::Build> {
    let rocket = crate::rocket();
    let config = rocket
        .figment()
        .clone()
        .merge((
            "databases.survey_app.url",
            format!("postgres://vscode:notsecure@db/{}", db_name),
        ))
        .merge(("databases.survey_app.pool_size", 1));
    return rocket.configure(config);
}

pub fn bench_rocket(db_name: &String) -> rocket::Rocket<rocket::Build> {
    let rocket = crate::rocket();
    let config = rocket
        .figment()
        .clone()
        .merge((
            "databases.survey_app.url",
            format!("postgres://vscode:notsecure@db/{}", db_name),
        ))
        .merge(("databases.survey_app.pool_size", 1))
        .merge(("secret_key", vec![12u8; 64]));
    return rocket.configure(config);
}

pub fn make_survey(client: &Client, token: &String) -> i32 {
    let response = client
        .post(uri!("/api", crate::survey::create_survey))
        .header(rocket::http::ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.clone()))
        .dispatch();
    response.into_json::<Survey>().unwrap().id
}

pub fn publish_survey(client: &Client, token: &String, survey_id: i32) {
    client
        .patch(uri!("/api", crate::survey::edit_survey(survey_id)).to_string())
        .header(rocket::http::ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.clone()))
        .body(
            serde_json::to_vec(&SurveyPatch {
                published: Some(true),
                ..Default::default()
            })
            .unwrap(),
        )
        .dispatch();
}
