use diesel::prelude::*;
use rocket::{http::Status, serde::json::Json, request::FromParam, form::{ValueField, FromFormField}};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    api::ApiErrorResponse,
    db::{
        models::{NewSurveyResponse, SurveyResponses, PatchSurveyResponse},
        Storage,
    },
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAccepted {
    responder_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum SurveyResponseError {
    #[error("Survey not found")]
    SurveyNotFound,
    #[error("Survey responder not found")]
    ResponderNotFound,
    #[error("Validation failed")]
    ValidationFailed,
    #[error("Unknown error")]
    Unknown,
}

impl From<SurveyResponseError> for ApiErrorResponse<SurveyResponseError> {
    fn from(value: SurveyResponseError) -> Self {
        let status = match &value {
            SurveyResponseError::SurveyNotFound => Status::NotFound,
            SurveyResponseError::ResponderNotFound => Status::NotFound,
            SurveyResponseError::ValidationFailed => Status::BadRequest,
            SurveyResponseError::Unknown => Status::InternalServerError,
        };
        ApiErrorResponse {
            status,
            message: value,
        }
    }
}

#[post("/survey/<survey_id>/respond", data = "<survey_response>")]
pub async fn create_survey_response(
    db: Storage,
    survey_id: i32,
    survey_response: Json<SurveyResponses>,
) -> Result<Json<ResponseAccepted>, ApiErrorResponse<SurveyResponseError>> {
    let uuid = db
        .run(move |conn| {
            let survey_response = survey_response.into_inner();
            let uuid = Uuid::new_v4();
            let new_survey_response = NewSurveyResponse {
                survey_id,
                responder_uuid: uuid,
                content: survey_response,
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
pub async fn edit_survey_response(db: Storage, survey_id: i32, survey_response: Json<SurveyResponses>, responder: Uuid) -> Result<(), ApiErrorResponse<SurveyResponseError>> {
    let survey_response = survey_response.into_inner();
    db.run(move |conn| {
        let patch_survey_response = PatchSurveyResponse {
            content: survey_response,
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
