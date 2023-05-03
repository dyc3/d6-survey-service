use diesel::prelude::*;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    api::{ApiErrorResponse, ApiOkCacheableResource},
    cache::{CacheCheck, Cacheable, RaceCheck},
    db::{
        models::{
            NewSurveyResponse, PatchSurveyResponse, Survey, SurveyResponse,
            SurveyResponseUpdateCheck, SurveyResponses,
        },
        Storage,
    },
    jwt::Claims,
    validate::{Validate, ValidationError},
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAccepted {
    #[typeshare(serialized_as = "String")]
    responder_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum SurveyResponseError {
    #[error("Data race")]
    RaceError,
    #[error("Survey not found")]
    SurveyNotFound,
    #[error("Survey not published")]
    SurveyNotPublished,
    #[error("Survey responder not found")]
    ResponderNotFound,
    #[error("Not survey owner")]
    NotSurveyOwner,
    #[error("Validation Error")]
    ValidationError(Vec<ValidationError>),
    #[error("Unknown error")]
    Unknown,
}

impl From<SurveyResponseError> for ApiErrorResponse<SurveyResponseError> {
    fn from(value: SurveyResponseError) -> Self {
        let status = match &value {
            SurveyResponseError::RaceError => Status::PreconditionFailed,
            SurveyResponseError::SurveyNotFound => Status::NotFound,
            SurveyResponseError::SurveyNotPublished => Status::Forbidden,
            SurveyResponseError::ResponderNotFound => Status::NotFound,
            SurveyResponseError::NotSurveyOwner => Status::Forbidden,
            SurveyResponseError::ValidationError(_) => Status::UnprocessableEntity,
            SurveyResponseError::Unknown => Status::InternalServerError,
        };
        ApiErrorResponse {
            status,
            message: value,
        }
    }
}

impl From<Vec<ValidationError>> for ApiErrorResponse<SurveyResponseError> {
    fn from(value: Vec<ValidationError>) -> Self {
        SurveyResponseError::ValidationError(value).into()
    }
}

async fn get_survey_from_db(db: &Storage, survey_id: i32) -> Result<Survey, SurveyResponseError> {
    let survey = crate::survey::get_survey_from_db(db, survey_id)
        .await
        .map_err(|_| SurveyResponseError::SurveyNotFound)?;

    if !survey.published {
        return Err(SurveyResponseError::SurveyNotPublished);
    }

    Ok(survey)
}

#[post("/survey/<survey_id>/respond", data = "<survey_response>")]
pub async fn create_survey_response(
    db: Storage,
    survey_id: i32,
    survey_response: Json<SurveyResponses>,
) -> Result<Json<ResponseAccepted>, ApiErrorResponse<SurveyResponseError>> {
    let survey = get_survey_from_db(&db, survey_id).await?;

    let survey_responses = survey_response.into_inner();
    (&survey.questions, &survey_responses).validate()?;

    let uuid = db
        .run(move |conn| {
            let uuid = Uuid::new_v4();
            let new_survey_response = NewSurveyResponse {
                survey_id,
                responder_uuid: uuid,
                content: survey_responses,
            };
            diesel::insert_into(crate::db::schema::responses::table)
                .values(&new_survey_response)
                .execute(conn)
                .map(|_| uuid)
        })
        .await
        .map_err(|e| {
            error!("{e:?}");
            SurveyResponseError::Unknown
        })?;

    Ok(Json(ResponseAccepted {
        responder_uuid: uuid,
    }))
}

#[patch("/survey/<survey_id>/respond?<responder>", data = "<survey_response>")]
pub async fn edit_survey_response(
    db: Storage,
    survey_id: i32,
    survey_response: Json<SurveyResponses>,
    responder: Uuid,
    race_check: Option<RaceCheck>,
) -> Result<Json<()>, ApiErrorResponse<SurveyResponseError>> {
    if let Some(race_check) = race_check {
        let old_response = db
            .run(move |conn| {
                crate::db::schema::responses::table
                    .select((crate::db::schema::responses::updated_at,))
                    .find(responder)
                    .first::<SurveyResponseUpdateCheck>(conn)
            })
            .await
            .map_err(|e| {
                error!("{e:?}");
                SurveyResponseError::Unknown
            })?;

        if !old_response.has_no_mid_air_collision(race_check) {
            return Err(SurveyResponseError::RaceError.into());
        }
    }

    let survey = get_survey_from_db(&db, survey_id).await?;

    let survey_responses = survey_response.into_inner();
    (&survey.questions, &survey_responses).validate()?;

    db.run(move |conn| {
        conn.build_transaction()
            .read_write()
            .run::<_, diesel::result::Error, _>(|conn| {
                let patch_survey_response = PatchSurveyResponse {
                    content: survey_responses,
                };
                crate::db::schema::responses::table
                    .for_update()
                    .filter(crate::db::schema::responses::survey_id.eq(survey_id))
                    .filter(crate::db::schema::responses::responder_uuid.eq(responder))
                    .limit(1)
                    .load::<SurveyResponse>(conn)?;
                diesel::update(crate::db::schema::responses::table)
                    .filter(crate::db::schema::responses::survey_id.eq(survey_id))
                    .filter(crate::db::schema::responses::responder_uuid.eq(responder))
                    .set(&patch_survey_response)
                    .execute(conn)?;
                Ok(())
            })
    })
    .await
    .map_err(|e| {
        error!("{e:?}");
        SurveyResponseError::Unknown
    })?;

    Ok(Json(()))
}

#[get("/survey/<survey_id>/respond?<responder>")]
pub async fn get_survey_response(
    db: Storage,
    survey_id: i32,
    responder: Uuid,
    cache_check: Option<CacheCheck>,
) -> Result<ApiOkCacheableResource<SurveyResponse>, ApiErrorResponse<SurveyResponseError>> {
    let survey_response = db
        .run(move |conn| {
            crate::db::schema::responses::table
                .filter(crate::db::schema::responses::survey_id.eq(survey_id))
                .filter(crate::db::schema::responses::responder_uuid.eq(responder))
                .first::<SurveyResponse>(conn)
        })
        .await
        .map_err(|e| {
            error!("{e:?}");
            SurveyResponseError::Unknown
        })?;

    if let Some(cache_check) = cache_check {
        if survey_response.is_cache_fresh(cache_check) {
            return Ok(ApiOkCacheableResource::NotModified);
        }
    }

    Ok(ApiOkCacheableResource::Ok(survey_response))
}

#[delete("/survey/<survey_id>/respond")]
pub async fn clear_survey_responses(
    db: Storage,
    survey_id: i32,
    claims: Claims,
) -> Result<Json<()>, ApiErrorResponse<SurveyResponseError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyResponseError::SurveyNotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyResponseError::NotSurveyOwner.into());
    }

    if !survey.published {
        return Err(SurveyResponseError::SurveyNotPublished.into());
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
        SurveyResponseError::Unknown
    })?;

    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::SurveyResponses;
    use crate::test_helpers::*;
    use rocket::local::blocking::Client;
    use std::collections::HashMap;

    #[test]
    fn test_clear_survey_responses() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let owner_token = create_test_user(&client);
            let survey_id = make_survey(&client, &owner_token);
            publish_survey(&client, &owner_token, survey_id);

            let map: HashMap<Uuid, crate::questions::Response> = HashMap::new();

            let response = client
                .post(uri!("/api", create_survey_response(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .body(serde_json::to_vec(&SurveyResponses(map)).unwrap())
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);

            // assert there is a response
            let response = client
                .get(uri!("/api", crate::survey::export::export_responses(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new(
                    "Authorization",
                    owner_token.clone(),
                ))
                .dispatch();
            assert_eq!(response.status(), rocket::http::Status::Ok);

            let csv = response.into_string().unwrap();
            assert_ne!(csv, "responder,created_at,updated_at\n");

            let response = client
                .delete(uri!("/api", clear_survey_responses(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new(
                    "Authorization",
                    owner_token.clone(),
                ))
                .dispatch();

            assert_eq!(response.status(), rocket::http::Status::Ok);

            // assert there are no responses
            let response = client
                .get(uri!("/api", crate::survey::export::export_responses(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", owner_token))
                .dispatch();
            assert_eq!(response.status(), rocket::http::Status::Ok);

            let csv = response.into_string().unwrap();
            assert_eq!(csv, "responder,created_at,updated_at\n");
        });
    }
}
