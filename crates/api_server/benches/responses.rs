use std::str::FromStr;

use api_server::db::models::{SurveyPatch, SurveyQuestions};
use api_server::questions::{Choice, QMultipleChoice, QRating, QText, SurveyQuestion};
use api_server::test_helpers::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rocket::local::blocking::Client;
use rocket::uri;
use uuid::Uuid;

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
            assert_eq!(resp.status(), rocket::http::Status::Ok);
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
                min_text: "Gross".to_string(),
                max_text: "Tasty".to_string(),
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
                min_text: "Boring".to_string(),
                max_text: "Fun".to_string(),
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

fn survey_response_stress(c: &mut Criterion) {
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
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            required: true,
            question: QRating {
                prompt: "Rate your food.".to_string(),
                description: "please".to_string(),
                max_rating: 5,
                min_text: "Gross".to_string(),
                max_text: "Tasty".to_string(),
            }
            .into(),
        },
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000003").unwrap(),
            required: true,
            question: QRating {
                prompt: "Rate your fun.".to_string(),
                description: "please".to_string(),
                max_rating: 5,
                min_text: "Boring".to_string(),
                max_text: "Fun".to_string(),
            }
            .into(),
        },
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000004").unwrap(),
            required: true,
            question: QMultipleChoice {
                prompt: "Rate your food.".to_string(),
                description: "please".to_string(),
                multiple: true,
                choices: (0u64..10)
                    .map(|n| Choice {
                        uuid: Uuid::from_u128(n.into()),
                        text: format!("Choice {n}"),
                    })
                    .collect(),
            }
            .into(),
        },
        SurveyQuestion {
            uuid: uuid::Uuid::from_str("00000000-0000-0000-0000-000000000005").unwrap(),
            required: true,
            question: QMultipleChoice {
                prompt: "Rate your fun.".to_string(),
                description: "please".to_string(),
                multiple: false,
                choices: (0u64..10)
                    .map(|n| Choice {
                        uuid: Uuid::from_u128(n.into()),
                        text: format!("Choice {n}"),
                    })
                    .collect(),
            }
            .into(),
        },
    ]
    .into();
    let survey_id = make_test_survey(&client, questions);

    c.bench_function("survey response stress test", |b| {
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
                        },
                        "00000000-0000-0000-0000-000000000002": {
                            "type": "Rating",
                            "content": {
                                "rating": 4
                            }
                        },
                        "00000000-0000-0000-0000-000000000003": {
                            "type": "Rating",
                            "content": {
                                "rating": 4
                            }
                        },
                        "00000000-0000-0000-0000-000000000004": {
                            "type": "MultipleChoice",
                            "content": {
                                "selected": ["00000000-0000-0000-0000-000000000000", "00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000002", "00000000-0000-0000-0000-000000000003", "00000000-0000-0000-0000-000000000004", "00000000-0000-0000-0000-000000000005", "00000000-0000-0000-0000-000000000006", "00000000-0000-0000-0000-000000000007", "00000000-0000-0000-0000-000000000008", "00000000-0000-0000-0000-000000000009"]
                            }
                        },
                        "00000000-0000-0000-0000-000000000005": {
                            "type": "MultipleChoice",
                            "content": {
                                "selected": ["00000000-0000-0000-0000-000000000000"]
                            }
                        }
                    }))
                    .unwrap(),
                )
                .dispatch();
            assert_eq!(resp.status(), rocket::http::Status::Ok);
            black_box(resp);
        });
    });

    drop_test_db(db_name)
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = survey_text_questions, survey_rating_questions, survey_response_stress
}
criterion_main!(benches);
