use diesel::prelude::*;
use rocket::{http::Status, response::status::Created, serde::json::Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    api::ApiErrorResponse,
    db::{
        models::{NewSurvey, Survey, SurveyPatch},
        schema, Storage,
    },
    jwt::Claims,
};

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SurveyError {
    #[error("Not published")]
    NotPublished,
    #[error("Not owner")]
    NotOwner,
    #[error("Not found")]
    NotFound,
    #[error("Internal error")]
    Unknown,
}

impl From<SurveyError> for ApiErrorResponse<SurveyError> {
    fn from(value: SurveyError) -> Self {
        let status = match &value {
            SurveyError::NotPublished => Status::Forbidden,
            SurveyError::NotOwner => Status::Forbidden,
            SurveyError::NotFound => Status::NotFound,
            SurveyError::Unknown => Status::InternalServerError,
        };
        ApiErrorResponse {
            status,
            message: value,
        }
    }
}

#[post("/survey/create")]
pub async fn create_survey(
    claims: Claims,
    db: Storage,
) -> Result<Created<Json<Survey>>, ApiErrorResponse<SurveyError>> {
    let new_survey = NewSurvey::new(claims.user_id());

    let mut surveys = db
        .run(move |conn| -> anyhow::Result<Vec<Survey>> {
            let surveys = diesel::insert_into(schema::surveys::table)
                .values(&new_survey)
                .get_results::<Survey>(conn)?;
            Ok(surveys)
        })
        .await
        .map_err(|e| {
            error!("{e:?}");
            SurveyError::Unknown
        })?;
    if surveys.len() != 1 {
        return Err(SurveyError::Unknown.into());
    }
    let survey = surveys.remove(0);

    let resource_uri = uri!(get_survey_auth(survey.id)).to_string();
    Ok(Created::new(resource_uri).body(Json(survey)))
}

#[get("/survey/<survey_id>")]
pub async fn get_survey_auth(
    survey_id: i32,
    db: Storage,
    claims: Claims,
) -> Result<Json<Survey>, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyError::NotOwner.into());
    }

    Ok(Json(survey))
}

#[get("/survey/<survey_id>", rank = 2)]
pub async fn get_survey(
    survey_id: i32,
    db: Storage,
) -> Result<Json<Survey>, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if !survey.published {
        return Err(SurveyError::NotPublished.into());
    }

    Ok(Json(survey))
}

#[patch("/survey/<survey_id>", data = "<new_survey>")]
pub async fn edit_survey(
    survey_id: i32,
    claims: Claims,
    db: Storage,
    new_survey: Json<SurveyPatch>,
) -> Result<(), ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyError::NotOwner.into());
    }

    // TODO: validate questions

    db.run(move |conn| -> anyhow::Result<()> {
        diesel::update(schema::surveys::table)
            .filter(schema::surveys::id.eq(survey_id))
            .set(new_survey.into_inner())
            .execute(conn)?;
        Ok(())
    })
    .await
    .map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;

    Ok(())
}

async fn get_survey_from_db(db: &Storage, survey_id: i32) -> anyhow::Result<Survey> {
    db.run(move |conn| {
        let survey = schema::surveys::dsl::surveys
            .find(survey_id)
            .first::<Survey>(conn)?;
        Ok(survey)
    })
    .await
}

#[cfg(test)]
mod tests {
    use std::panic;

    use crate::db::models::SurveyQuestions;

    use super::*;
    use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
    use jsonwebtoken::EncodingKey;
    use rocket::local::blocking::Client;

    fn create_db_for_tests() -> String {
        let db_name = format!("survey_app_test_{}", uuid::Uuid::new_v4()).replace("-", "_");
        let mut conn = PgConnection::establish("postgres://vscode:notsecure@db/survey_app")
            .expect("Failed to connect to database");
        sql_query(format!("CREATE DATABASE {db_name}"))
            .execute(&mut conn)
            .expect("Failed to create test database");
        db_name.to_string()
    }

    fn drop_test_db(db_name: String) {
        let mut conn = PgConnection::establish("postgres://vscode:notsecure@db/survey_app")
            .expect("Failed to connect to database");
        sql_query(format!("DROP DATABASE IF EXISTS {db_name}"))
            .execute(&mut conn)
            .expect("Failed to drop test database");
    }

    fn create_test_user(client: &Client) -> String {
        let response = client
            .post(uri!("/api", crate::user::register_user))
            .header(rocket::http::ContentType::JSON)
            .body(r#"{"username": "test", "password": "test"}"#)
            .dispatch();
        format!(
            "Bearer {}",
            response
                .into_json::<crate::user::UserToken>()
                .unwrap()
                .token
        )
    }

    fn make_jwt(client: &Client, user_id: i32) -> String {
        let key =
            EncodingKey::from_secret(client.rocket().config().secret_key.to_string().as_bytes());
        let claims = Claims::new(user_id);
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();
        "Bearer ".to_string() + &token
    }

    fn run_test_with_db<T>(test: T) -> ()
    where
        T: FnOnce(&String) -> () + panic::UnwindSafe,
    {
        let db_name = create_db_for_tests();
        let result = panic::catch_unwind(|| test(&db_name));
        drop_test_db(db_name);
        assert!(result.is_ok())
    }

    fn test_rocket(db_name: &String) -> rocket::Rocket<rocket::Build> {
        let rocket = crate::rocket();
        let config = rocket
            .figment()
            .clone()
            .merge((
                "databases.survey_app_test.url",
                format!("postgres://vscode:notsecure@db/{}", db_name),
            ))
            .merge(("databases.survey_app_test.pool_size", 1));
        return rocket.configure(config);
    }

    #[test]
    fn test_create_survey() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let response = client
                .post(uri!("/api", create_survey))
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Created);
        });
    }

    #[test]
    fn test_get_survey() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let response = client
                .post(uri!("/api", create_survey))
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Created);

            let survey_id = response.into_json::<Survey>().unwrap().id;

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
        });
    }

    #[test]
    fn test_edit_survey() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let response = client
                .post(uri!("/api", create_survey))
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Created);

            let survey_id = response.into_json::<Survey>().unwrap().id;

            let response = client
                .patch(uri!("/api", edit_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .body(
                    serde_json::to_vec(&SurveyPatch {
                        title: Some("test".to_owned()),
                        description: Some(":)".to_owned()),
                        published: Some(true),
                        questions: Some(SurveyQuestions(vec![])),
                    })
                    .unwrap(),
                )
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
            let survey = response.into_json::<Survey>().unwrap();
            assert_eq!(survey.title, "test");
            assert_eq!(survey.description, ":)");
            assert_eq!(survey.published, true);
            assert_eq!(survey.questions.0.len(), 0);
        });
    }
}
