use diesel::prelude::*;
use rocket::{http::Status, serde::json::Json};
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
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Internal error")]
    Unknown,
}

impl From<SurveyError> for ApiErrorResponse<SurveyError> {
    fn from(value: SurveyError) -> Self {
        let status = match &value {
            SurveyError::Unauthorized => Status::Unauthorized,
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
) -> Result<Json<Survey>, ApiErrorResponse<SurveyError>> {
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

    Ok(Json(survey))
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
        return Err(SurveyError::Unauthorized.into());
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
        return Err(SurveyError::Unauthorized.into());
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
        return Err(SurveyError::Unauthorized.into());
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
