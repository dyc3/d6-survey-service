use std::borrow::Cow;
use std::io::Cursor;

use diesel::prelude::*;
use rocket::http::{ContentType, Header, Status};
use rocket::response::Responder;

use crate::api::ApiErrorResponse;
use crate::db::models::{Survey, SurveyResponse};
use crate::db::Storage;
use crate::jwt::Claims;
use crate::questions::{Question, Response};
use crate::survey::{get_survey_from_db, SurveyError};

#[get("/survey/<survey_id>/export")]
pub async fn export_responses(
    survey_id: i32,
    claims: Claims,
    db: Storage,
) -> Result<ExportedResults, ApiErrorResponse<SurveyError>> {
    let survey = get_survey_from_db(&db, survey_id).await.map_err(|e| {
        error!("{e:?}");
        SurveyError::NotFound
    })?;

    if survey.owner_id != claims.user_id() {
        return Err(SurveyError::NotOwner.into());
    }

    let responses: Vec<SurveyResponse> = db
        .run(move |conn| {
            crate::db::schema::responses::dsl::responses
                .filter(crate::db::schema::responses::survey_id.eq(survey_id))
                .load::<SurveyResponse>(conn)
        })
        .await
        .map_err(|e| {
            error!("{e:?}");
            SurveyError::Unknown
        })?;

    let mut wtr = csv::Writer::from_writer(vec![]);
    write_csv_rows::<Vec<u8>>(&mut wtr, &survey, &responses).map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;

    let rendered_csv = wtr.into_inner().map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;
    let rendered_csv = String::from_utf8(rendered_csv).map_err(|e| {
        error!("{e:?}");
        SurveyError::Unknown
    })?;
    Ok(ExportedResults {
        survey,
        csv: rendered_csv,
    })
}

fn build_csv_header(survey: &Survey) -> Vec<String> {
    let mut header = Vec::with_capacity(survey.questions.len() + 3);
    header.push("responder".to_string());
    header.push("created_at".to_string());
    header.push("updated_at".to_string());
    for question in survey.questions.iter() {
        let prompt = match &question.question {
            Question::Text(q) => &q.prompt,
            Question::MultipleChoice(q) => &q.prompt,
            Question::Rating(q) => &q.prompt,
        };
        header.push(prompt.clone()); // TODO: get rid of clone
    }
    header
}

fn write_csv_rows<C: std::io::Write>(
    wtr: &mut csv::Writer<C>,
    survey: &Survey,
    responses: &Vec<SurveyResponse>,
) -> anyhow::Result<()> {
    let header = build_csv_header(survey);
    wtr.write_record(&header)?;

    for response in responses {
        // TODO: use write field instead
        let mut row: Vec<String> = Vec::with_capacity(survey.questions.len() + 3);
        row.push(response.responder_uuid.to_string());
        row.push(response.created_at.to_string());
        row.push(response.updated_at.to_string());

        for question in survey.questions.iter() {
            let Some(qresponse) = response.content.0.get(&question.uuid) else {
                row.push("".to_string());
                continue;
            };
            let value = match qresponse {
                Response::Text(r) => Cow::from(&r.text),
                Response::MultipleChoice(r) => {
                    let Question::MultipleChoice(q) = &question.question else {
                        anyhow::bail!("question type mismatch");
                    };

                    r.selected
                        .iter()
                        .filter_map(|choice_id| {
                            let Some(choice) = q.choices.iter().find(|c| c.uuid == *choice_id) else {
                                return None;
                            };
                            Some(choice.text.clone())
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                        .to_owned().into()
                }
                Response::Rating(r) => r.rating.to_string().into(),
            };
            row.push(value.to_string());
        }

        wtr.write_record(&row)?;
    }
    Ok(())
}

pub struct ExportedResults {
    survey: Survey,
    csv: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ExportedResults {
    fn respond_to(self, req: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        // TODO: make file name the title of the survey with non alphanumeric characters replaced with underscores
        rocket::Response::build()
            .status(Status::Ok)
            .header(ContentType::new("text", "csv"))
            .header(Header::new(
                "Content-Disposition",
                "attachment; filename=\"results.csv\"",
            ))
            .sized_body(self.csv.len(), Cursor::new(self.csv))
            .ok()
    }
}
