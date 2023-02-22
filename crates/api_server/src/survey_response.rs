use rocket::serde::json::Json;
use uuid::Uuid;

use crate::db::models::SurveyResponses;

#[post("/survey/<survey_id>/respond", data = "<survey_response>")]
fn create_survey_response(survey_id: i32, survey_response: Json<SurveyResponses>) {

}

#[patch("/survey/<survey_id>/respond?responder=<responder_uuid>", data = "<survey_response>")]
fn edit_survey_response(survey_id: i32, survey_response: Json<SurveyResponses>, responder_uuid: Uuid) {

}