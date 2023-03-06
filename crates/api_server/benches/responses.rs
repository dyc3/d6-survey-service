use std::str::FromStr;

use api_server::db::models::{SurveyPatch, SurveyQuestions};
use api_server::questions::{QRating, QText, SurveyQuestion};
use api_server::test_helpers::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rocket::local::blocking::Client;
use rocket::uri;

fn make_test_survey(client: &Client, questions: SurveyQuestions) -> i32 {
    let token = create_test_user(client);
    let survey_id = make_survey(client, &token);
    let patch = SurveyPatch {
        published: Some(true),
        questions: Some(questions),
        ..Default::default()
    };
    client
        .patch(uri!("/api", api_server::survey::edit_survey(survey_id)))
        .header(rocket::http::ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token))
        .body(serde_json::to_vec(&patch).unwrap())
        .dispatch();
    survey_id
}

fn survey_text_questions(c: &mut Criterion) {
    let db_name = create_db_for_tests();
    let client = Client::untracked(bench_rocket(&db_name)).expect("valid rocket instance");

    let questions: SurveyQuestions = vec![
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            required: true,
            question: QText {
                prompt: "What is your name?".to_string(),
                description: "Please enter your name".to_string(),
                multiline: false,
            }
            .into(),
        },
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            required: true,
            question: QText {
                prompt: "Describe your shoes".to_string(),
                description: "please".to_string(),
                multiline: true,
            }
            .into(),
        },
    ]
    .into();
    let survey_id = make_test_survey(&client, questions);

    c.bench_function("survey with text questions", |b| {
        b.iter(|| {
            let resp = client
                .post(uri!(
                    "/api",
                    api_server::survey_response::create_survey_response(survey_id)
                ))
                .header(rocket::http::ContentType::JSON)
                .body(
                    serde_json::to_vec(&serde_json::json!({
                        "00000000-0000-0000-0000-000000000000": {
                            "type": "Text",
                            "content": {
                                "text": "test"
                            }
                        },
                        "00000000-0000-0000-0000-000000000001": {
                            "type": "Text",
                            "content": {
                                "text": "test"
                            }
                        }
                    }))
                    .unwrap(),
                )
                .dispatch();
            black_box(resp);
        });
    });

    drop_test_db(db_name)
}

fn survey_rating_questions(c: &mut Criterion) {
    let db_name = create_db_for_tests();
    let client = Client::untracked(bench_rocket(&db_name)).expect("valid rocket instance");

    let questions: SurveyQuestions = vec![
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            required: true,
            question: QRating {
                prompt: "Rate your food.".to_string(),
                description: "please".to_string(),
                max_rating: 5,
            }
            .into(),
        },
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            required: true,
            question: QRating {
                prompt: "Rate your fun.".to_string(),
                description: "please".to_string(),
                max_rating: 5,
            }
            .into(),
        },
    ]
    .into();
    let survey_id = make_test_survey(&client, questions);

    c.bench_function("survey with rating questions", |b| {
        b.iter(|| {
            let resp = client
                .post(uri!(
                    "/api",
                    api_server::survey_response::create_survey_response(survey_id)
                ))
                .header(rocket::http::ContentType::JSON)
                .body(
                    serde_json::to_vec(&serde_json::json!({
                        "00000000-0000-0000-0000-000000000000": {
                            "type": "Rating",
                            "content": {
                                "rating": 4
                            }
                        },
                        "00000000-0000-0000-0000-000000000001": {
                            "type": "Rating",
                            "content": {
                                "rating": 4
                            }
                        }
                    }))
                    .unwrap(),
                )
                .dispatch();
            black_box(resp);
        });
    });

    drop_test_db(db_name)
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = survey_text_questions, survey_rating_questions
}
criterion_main!(benches);
