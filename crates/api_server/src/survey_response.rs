use rocket::serde::json::Json;

use crate::questions::SurveyResponse;

#[post("/survey/<survey_id>/respond", data = "<survey_response>")]
fn create_survey_response(survey_id: i32, survey_response: Json<SurveyResponse>) {

}

#[patch("/survey/<survey_id>/respond", data = "<survey_response>")]
fn edit_survey_response(survey_id: i32, survey_response: Json<SurveyResponse>) {

}