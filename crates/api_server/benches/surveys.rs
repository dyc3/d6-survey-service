use api_server::db::models::SurveyPatch;
use api_server::questions::{Choice, QMultipleChoice, QRating, QText, SurveyQuestion};
use api_server::test_helpers::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rocket::local::blocking::Client;
use rocket::uri;

fn get_survey(c: &mut Criterion) {
    let questions = vec![
        QText {
            prompt: "What is your name?".to_string(),
            description: "Please enter your name".to_string(),
            multiline: false,
        }
        .into(),
        QRating {
            prompt: "Rate the foobar".to_string(),
            description: "1 is the worst, 5 is the best".to_string(),
            max_rating: 5,
        }
        .into(),
        QMultipleChoice {
            prompt: "Pick one".to_string(),
            description: "yes".to_string(),
            multiple: false,
            choices: vec![
                Choice {
                    uuid: uuid::Uuid::new_v4(),
                    text: "amogus".to_string(),
                },
                Choice {
                    uuid: uuid::Uuid::new_v4(),
                    text: "fortnight".to_string(),
                },
            ],
        }
        .into(),
        QText {
            prompt: "What is your name?".to_string(),
            description: "Please enter your name".to_string(),
            multiline: true,
        }
        .into(),
        QRating {
            prompt: "Rate the foobar".to_string(),
            description: "1 is the worst, 5 is the best".to_string(),
            max_rating: 5,
        }
        .into(),
    ]
    .into_iter()
    .map(|q| SurveyQuestion {
        uuid: uuid::Uuid::new_v4(),
        required: false,
        question: q,
    })
    .collect::<Vec<_>>();

    let db_name = create_db_for_tests();
    let client = Client::untracked(bench_rocket(&db_name)).expect("valid rocket instance");
    let token = create_test_user(&client);
    fn make_test_survey(
        client: &Client,
        token: &String,
        questions: &Vec<SurveyQuestion>,
        mut amount: usize,
    ) -> i32 {
        let survey_id = make_survey(&client, &token);
        let mut q = vec![];
        while amount > questions.len() {
            q.extend(questions.clone());
            amount -= questions.len();
        }
        q.extend(questions[..amount].to_vec());
        client
            .patch(uri!("/api", api_server::survey::edit_survey(survey_id)).to_string())
            .header(rocket::http::ContentType::JSON)
            .header(rocket::http::Header::new("Authorization", token.clone()))
            .body(
                serde_json::to_vec(&SurveyPatch {
                    title: Some("Benchmark Survey".to_string()),
                    published: Some(true),
                    questions: Some(q.into()),
                    ..Default::default()
                })
                .unwrap(),
            )
            .dispatch();
        survey_id
    }

    for survey_size in [2, 4, 8, 16, 32].iter() {
        let survey_id = make_test_survey(&client, &token, &questions, *survey_size);
        c.bench_with_input(
            BenchmarkId::new("get_survey", survey_size),
            survey_size,
            |b, _| {
                b.iter(|| {
                    let resp = client
                        .get(uri!("/api", api_server::survey::get_survey(survey_id)).to_string())
                        .dispatch();
                    black_box(resp);
                })
            },
        );
    }

    drop_test_db(db_name);
}

fn patch_survey(c: &mut Criterion) {
    let questions = vec![
        QText {
            prompt: "What is your name?".to_string(),
            description: "Please enter your name".to_string(),
            multiline: false,
        }
        .into(),
        QRating {
            prompt: "Rate the foobar".to_string(),
            description: "1 is the worst, 5 is the best".to_string(),
            max_rating: 5,
        }
        .into(),
        QMultipleChoice {
            prompt: "Pick one".to_string(),
            description: "yes".to_string(),
            multiple: false,
            choices: vec![
                Choice {
                    uuid: uuid::Uuid::new_v4(),
                    text: "amogus".to_string(),
                },
                Choice {
                    uuid: uuid::Uuid::new_v4(),
                    text: "fortnight".to_string(),
                },
            ],
        }
        .into(),
        QText {
            prompt: "What is your name?".to_string(),
            description: "Please enter your name".to_string(),
            multiline: true,
        }
        .into(),
        QRating {
            prompt: "Rate the foobar".to_string(),
            description: "1 is the worst, 5 is the best".to_string(),
            max_rating: 5,
        }
        .into(),
    ]
    .into_iter()
    .map(|q| SurveyQuestion {
        uuid: uuid::Uuid::new_v4(),
        required: false,
        question: q,
    })
    .collect::<Vec<_>>();

    let db_name = create_db_for_tests();
    let client = Client::untracked(bench_rocket(&db_name)).expect("valid rocket instance");
    let token = create_test_user(&client);
    fn make_test_survey(
        client: &Client,
        token: &String,
        questions: &Vec<SurveyQuestion>,
        mut amount: usize,
    ) -> (i32, Vec<u8>) {
        let survey_id = make_survey(&client, &token);
        let mut q = vec![];
        while amount > questions.len() {
            q.extend(questions.clone());
            amount -= questions.len();
        }
        q.extend(questions[..amount].to_vec());

        let body = serde_json::to_vec(&SurveyPatch {
            title: Some("Benchmark Survey".to_string()),
            published: Some(true),
            questions: Some(q.into()),
            ..Default::default()
        })
        .unwrap();
        (survey_id, body)
    }

    for survey_size in [2, 4, 8, 16, 32].iter() {
        let (survey_id, body) = make_test_survey(&client, &token, &questions, *survey_size);

        c.bench_with_input(
            BenchmarkId::new("patch_survey", survey_size),
            survey_size,
            |b, _| {
                b.iter(|| {
                    let resp = client
                        .patch(uri!("/api", api_server::survey::edit_survey(survey_id)).to_string())
                        .header(rocket::http::ContentType::JSON)
                        .header(rocket::http::Header::new("Authorization", token.clone()))
                        .body(body.clone())
                        .dispatch();
                    black_box(resp);
                })
            },
        );
    }

    drop_test_db(db_name);
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = get_survey, patch_survey
}
criterion_main!(benches);
