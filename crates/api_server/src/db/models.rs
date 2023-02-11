use serde::{Serialize, Deserialize};

use crate::{db::schema::users, questions::SurveyQuestion};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}

#[typeshare]
#[derive(Queryable, Serialize, Deserialize)]
pub struct Survey {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub published: bool,
    pub owner_id: i32,
    pub questions: Vec<SurveyQuestion>,
}

#[derive(Insertable)]
#[diesel(table_name=surveys)]
pub struct NewSurvey {
    owner_id: i32,
}
