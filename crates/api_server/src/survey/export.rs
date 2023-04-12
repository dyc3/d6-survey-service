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

fn write_csv_rows<C: std::io::Write>(
    wtr: &mut csv::Writer<C>,
    survey: &Survey,
    responses: &Vec<SurveyResponse>,
) -> anyhow::Result<()> {
    // write header
    wtr.write_field("responder")?;
    wtr.write_field("created_at")?;
    wtr.write_field("updated_at")?;
    for question in survey.questions.iter() {
        let prompt = match &question.question {
            Question::Text(q) => &q.prompt,
            Question::MultipleChoice(q) => &q.prompt,
            Question::Rating(q) => &q.prompt,
        };
        wtr.write_field(prompt)?;
    }
    wtr.write_record(None::<&[u8]>)?;

    // write rows
    for response in responses {
        wtr.write_field(response.responder_uuid.to_string())?;
        wtr.write_field(response.created_at.to_string())?;
        wtr.write_field(response.updated_at.to_string())?;

        for question in survey.questions.iter() {
            let Some(qresponse) = response.content.0.get(&question.uuid) else {
                wtr.write_field("")?;
                continue;
            };
            match qresponse {
                Response::Text(r) => {
                    wtr.write_field(&r.text)?;
                },
                Response::MultipleChoice(r) => {
                    let Question::MultipleChoice(q) = &question.question else {
                        anyhow::bail!("question type mismatch");
                    };

                    let selected = r.selected
                        .iter()
                        .filter_map(|choice_id| {
                            let Some(choice) = q.choices.iter().find(|c| c.uuid == *choice_id) else {
                                return None;
                            };
                            Some(choice.text.clone())
                        })
                        .collect::<Vec<String>>().join(",").to_string();
                    wtr.write_field(&selected)?;
                }
                Response::Rating(r) => {
                    wtr.write_field(r.rating.to_string())?;
                },
            }
        }
        wtr.write_record(None::<&[u8]>)?;
    }
    Ok(())
}

pub struct ExportedResults {
    survey: Survey,
    csv: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ExportedResults {
    fn respond_to(self, _req: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let filename = format!(
            "results_{}.csv",
            self.survey
                .title
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() {
                        c.to_lowercase().next().unwrap()
                    } else {
                        '_'
                    }
                })
                .collect::<String>()
        );

        rocket::Response::build()
            .status(Status::Ok)
            .header(ContentType::new("text", "csv"))
            .header(Header::new(
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", filename),
            ))
            .sized_body(self.csv.len(), Cursor::new(self.csv))
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use crate::{
        db::models::{SurveyPatch, SurveyQuestions},
        questions::{Choice, QMultipleChoice, QRating, QText, Question, SurveyQuestion},
        test_helpers::*,
    };
    use rocket::local::blocking::Client;
    use uuid::Uuid;

    #[test]
    fn csv_export() {
        run_test_with_db(|db_name| {
            let client = Client::tracked(test_rocket(db_name)).expect("valid rocket instance");

            let token = create_test_user(&client);
            let survey_id = make_survey(&client, &token);

            let response = client
                .patch(uri!("/api", crate::survey::edit_survey(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .body(
                    serde_json::to_vec(&SurveyPatch {
                        title: Some("test".to_owned()),
                        description: None,
                        published: Some(true),
                        questions: Some(SurveyQuestions(vec![
                            SurveyQuestion {
                                uuid: Uuid::from_str("00000000-0000-0000-0000-000000000000")
                                    .unwrap(),
                                question: Question::MultipleChoice(QMultipleChoice {
                                    prompt: "test".to_owned(),
                                    description: "".to_owned(),
                                    choices: vec![
                                        Choice {
                                            uuid: Uuid::from_str(
                                                "00000000-0000-0000-0000-000000000000",
                                            )
                                            .unwrap(),
                                            text: "foo".to_owned(),
                                        },
                                        Choice {
                                            uuid: Uuid::from_str(
                                                "00000000-0000-0000-0000-000000000001",
                                            )
                                            .unwrap(),
                                            text: "bar".to_owned(),
                                        },
                                    ],
                                    multiple: true,
                                }),
                                required: true,
                            },
                            SurveyQuestion {
                                uuid: Uuid::from_str("00000000-0000-0000-0000-000000000001")
                                    .unwrap(),
                                question: Question::Rating(QRating {
                                    prompt: "How much do you like this?".to_owned(),
                                    description: "".to_owned(),
                                    max_rating: 10,
                                }),
                                required: true,
                            },
                            SurveyQuestion {
                                uuid: Uuid::from_str("00000000-0000-0000-0000-000000000002")
                                    .unwrap(),
                                question: Question::Text(QText {
                                    prompt: "Anything else?".to_owned(),
                                    description: "".to_owned(),
                                    multiline: false,
                                }),
                                required: true,
                            },
                        ])),
                    })
                    .unwrap(),
                )
                .dispatch();
            assert_eq!(response.status(), rocket::http::Status::Ok);

            let response = client
                .post(uri!("/api", crate::survey_response::create_survey_response(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .body(
                    serde_json::to_vec(&serde_json::json!({
                        "00000000-0000-0000-0000-000000000000": {
                            "type": "MultipleChoice",
                            "content": {
                                "selected": [
                                    "00000000-0000-0000-0000-000000000000",
                                    "00000000-0000-0000-0000-000000000001",
                                ],
                            },
                        },
                        "00000000-0000-0000-0000-000000000001": {
                            "type": "Rating",
                            "content": {
                                "rating": 8
                            },
                        },
                        "00000000-0000-0000-0000-000000000002": {
                            "type": "Text",
                            "content": {
                                "text": "test"
                            },
                        }
                    }))
                    .unwrap(),
                )
                .dispatch();
            assert_eq!(response.status(), rocket::http::Status::Ok);

            let response = client
                .get(uri!("/api", export_responses(survey_id)).to_string())
                .header(rocket::http::ContentType::JSON)
                .header(rocket::http::Header::new("Authorization", token.clone()))
                .dispatch();
            assert_eq!(response.status(), rocket::http::Status::Ok);
            assert_eq!(response.content_type(), Some(rocket::http::ContentType::new("text", "csv")));
            assert_eq!(response.headers().get_one("Content-Disposition"), Some("attachment; filename=\"results_test.csv\""));
            let csv = response.into_string().unwrap();
            // a better assertion would be a regex, but im lazy and this is fine
            assert!(csv.starts_with("responder,created_at,updated_at,test,How much do you like this?,Anything else?\n"), "csv: {}", csv);
            assert!(csv.ends_with("\"foo,bar\",8,test\n"), "csv: {}", csv);
        });
    }
}
