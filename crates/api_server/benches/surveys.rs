use criterion::{black_box, criterion_group, criterion_main, Criterion};
use api_server::questions::{SurveyQuestion, QText, QRating, QMultipleChoice};
use api_server::test_helpers::*;
use api_server::db::models::SurveyPatch;
use rocket::local::blocking::Client;
use rocket::uri;


fn get_survey(c: &mut Criterion) {
    let questions = vec![
        QText {
            prompt: "What is your name?".to_string(),
            description: "Please enter your name".to_string(),
            multiline: false,
        }.into(),
        QRating {
            prompt: "Rate the foobar".to_string(),
            description: "1 is the worst, 5 is the best".to_string(),
            max_rating: 5
        }.into(),
        QMultipleChoice {
            prompt: "Pick one".to_string(),
            description: "yes".to_string(),
        }.into(),
        QText {
            prompt: "What is your name?".to_string(),
            description: "Please enter your name".to_string(),
            multiline: true,
        }.into(),
        QRating {
            prompt: "Rate the foobar".to_string(),
            description: "1 is the worst, 5 is the best".to_string(),
            max_rating: 5
        }.into(),
    ].into_iter().map(|q| {
        SurveyQuestion {
            uuid: uuid::Uuid::new_v4(),
            required: false,
            question: q,
        }
    }).collect::<Vec<_>>();

    let db_name = create_db_for_tests();
    let client = Client::untracked(bench_rocket(&db_name)).expect("valid rocket instance");
    let token = create_test_user(&client);
    fn make_test_survey(client: &Client, token: &String, questions: &Vec<SurveyQuestion>, mut amount: usize) -> i32 {
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
        c.bench_function(&format!("get survey: {} questions", survey_size), |b| b.iter(|| {
            let resp = client.get(uri!("/api", api_server::survey::get_survey(survey_id)).to_string())
                .dispatch();
            black_box(resp);
        }));
    }

    drop_test_db(db_name);
}

criterion_group!(benches, get_survey);
criterion_main!(benches);