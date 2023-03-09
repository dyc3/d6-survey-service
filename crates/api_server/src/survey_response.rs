use diesel::prelude::*;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    api::{ApiErrorResponse, ApiOkCacheableResource},
    db::{
        models::{NewSurveyResponse, PatchSurveyResponse, Survey, SurveyResponse, SurveyResponses, SurveyResponseUpdatedAt},
        Storage,
    }, cache::{RaceCheck, CacheCheck, Cacheable},
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
) -> Result<(), ApiErrorResponse<SurveyResponseError>> {
    if let Some(race_check) = race_check {
        let old_response = db
            .run(move |conn| {
                crate::db::schema::responses::table
                    .select((crate::db::schema::responses::updated_at,))
                    .find(responder)
                    .first::<SurveyResponseUpdatedAt>(conn)
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
        let patch_survey_response = PatchSurveyResponse {
            content: survey_responses,
        };
        diesel::update(crate::db::schema::responses::table)
            .filter(crate::db::schema::responses::survey_id.eq(survey_id))
            .filter(crate::db::schema::responses::responder_uuid.eq(responder))
            .set(&patch_survey_response)
            .execute(conn)
    })
    .await
    .map_err(|e| {
        error!("{e:?}");
        SurveyResponseError::Unknown
    })?;

    Ok(())
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
