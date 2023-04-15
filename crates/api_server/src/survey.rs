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
    validate::{Validate, ValidationError},
};

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum SurveyError {
    #[error("Can't edit questions on a published survey")]
    CantEditPublished,
    #[error("Not published")]
    NotPublished,
    #[error("Not owner")]
    NotOwner,
    #[error("Not found")]
    NotFound,
    #[error("Validation error")]
    ValidationError(Vec<ValidationError>),
    #[error("Internal error")]
    Unknown,
}

impl From<SurveyError> for ApiErrorResponse<SurveyError> {
    fn from(value: SurveyError) -> Self {
        let status = match &value {
            SurveyError::CantEditPublished => Status::Forbidden,
            SurveyError::NotPublished => Status::Forbidden,
            SurveyError::NotOwner => Status::Forbidden,
            SurveyError::NotFound => Status::NotFound,
            SurveyError::ValidationError(_) => Status::UnprocessableEntity,
            SurveyError::Unknown => Status::InternalServerError,
        };
        ApiErrorResponse {
            status,
            message: value,
        }
    }
}

impl From<Vec<ValidationError>> for ApiErrorResponse<SurveyError> {
    fn from(value: Vec<ValidationError>) -> Self {
        SurveyError::ValidationError(value).into()
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
    claims: Option<Claims>,
) -> Result<Json<Survey>, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if let Some(claims) = claims {
        if survey.owner_id != claims.user_id() && !survey.published {
            return Err(SurveyError::NotOwner.into());
        }
    } else if !survey.published {
        return Err(SurveyError::NotPublished.into());
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
) -> Result<Json<()>, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyError::NotOwner.into());
    }

    if survey.published && new_survey.questions.is_some() {
        return Err(SurveyError::CantEditPublished.into());
    }

    new_survey.validate()?;

    db.run(move |conn| -> anyhow::Result<()> {
        conn.build_transaction()
            .read_write()
            .run::<_, diesel::result::Error, _>(|conn| {
                schema::surveys::table
                    .for_update()
                    .find(survey_id)
                    .load::<Survey>(conn)?;
                diesel::update(schema::surveys::table)
                    .filter(schema::surveys::id.eq(survey_id))
                    .set(new_survey.into_inner())
                    .execute(conn)?;
                Ok(())
            })?;
        Ok(())
    })
    .await
    .map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;

    Ok(Json(()))
}

#[delete("/survey/<survey_id>")]
pub async fn delete_survey(
    survey_id: i32,
    claims: Claims,
    db: Storage,
) -> Result<Json<()>, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyError::NotOwner.into());
    }

    db.run(move |conn| -> anyhow::Result<()> {
        diesel::delete(schema::surveys::table)
            .filter(schema::surveys::id.eq(survey_id))
            .execute(conn)?;
        Ok(())
    })
    .await
    .map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;

    Ok(Json(()))
}

#[delete("/survey/<survey_id>/respond")]
pub async fn clear_survey_responses(
    db: Storage,
    survey_id: i32,
    claims: Claims,
) -> Result<Json<()>, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyError::NotOwner.into());
    }

    if !survey.published {
        return Err(SurveyError::NotPublished.into());
    }

    db.run(move |conn| -> anyhow::Result<()> {
        diesel::delete(crate::db::schema::responses::table)
            .filter(crate::db::schema::responses::survey_id.eq(survey_id))
            .execute(conn)?;
        Ok(())
    })
    .await
    .map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;

    Ok(Json(()))
}

pub(crate) async fn get_survey_from_db(db: &Storage, survey_id: i32) -> anyhow::Result<Survey> {
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
    use crate::db::models::SurveyQuestions;

    use super::*;
    use crate::test_helpers::*;
    use rocket::local::blocking::Client;

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
    fn test_get_survey_owner_unpublished() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
        });
    }

    #[test]
    fn test_get_survey_anonymous_published() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            publish_survey(&client, &token, survey_id);

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
        });
    }

    #[test]
    fn test_get_survey_forbidden_unpublished_not_owner() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            let token = make_jwt(&client, 58008);

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Forbidden);
        });
    }

    #[test]
    fn test_get_survey_forbidden_unpublished_anonymous() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Forbidden);
        });
    }

    #[test]
    fn test_get_survey_not_owner_but_published() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            publish_survey(&client, &token, survey_id);

            let token = make_jwt(&client, 58008);

            let response = client
                .get(uri!("/api", get_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
        });
    }

    #[test]
    fn test_edit_survey() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);

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
                .header(rocket::http::Header::new("Authorization", token))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
            let survey = response.into_json::<Survey>().unwrap();
            assert_eq!(survey.title, "test");
            assert_eq!(survey.description, ":)");
            assert!(survey.published);
            assert_eq!(survey.questions.0.len(), 0);
        });
    }

    #[test]
    fn test_edit_survey_owner_published() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            publish_survey(&client, &token, survey_id);

            let response = client
                .patch(uri!("/api", edit_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .body(
                    serde_json::to_vec(&SurveyPatch {
                        questions: Some(SurveyQuestions(vec![])),
                        ..Default::default()
                    })
                    .unwrap(),
                )
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Forbidden);
        });
    }

    #[test]
    fn test_edit_survey_owner_published_unpublish() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            publish_survey(&client, &token, survey_id);

            let response = client
                .patch(uri!("/api", edit_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .body(
                    serde_json::to_vec(&SurveyPatch {
                        published: Some(false),
                        ..Default::default()
                    })
                    .unwrap(),
                )
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);
        });
    }

    #[test]
    fn test_edit_survey_not_owner() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            publish_survey(&client, &token, survey_id);

            let token = make_jwt(&client, 58008);

            let response = client
                .patch(uri!("/api", edit_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token))
                .body(
                    serde_json::to_vec(&SurveyPatch {
                        questions: Some(SurveyQuestions(vec![])),
                        ..Default::default()
                    })
                    .unwrap(),
                )
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Forbidden);
        });
    }

    #[test]
    fn test_clear_survey_responses() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);
            publish_survey(&client, &token, survey_id);

            let token = make_jwt(&client, 58008);

            
        });
    }
}
