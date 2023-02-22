use std::{io::Write, collections::HashMap};

use diesel::{
    deserialize::FromSql,
    pg::{Pg, PgValue},
    serialize::ToSql,
    sql_types::Jsonb,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::schema::{surveys, users, responses},
    questions::{SurveyQuestion},
};

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
    pub questions: SurveyQuestions,
    #[typeshare(serialized_as = "String")]
    pub created_at: chrono::NaiveDateTime,
    #[typeshare(serialized_as = "String")]
    pub updated_at: chrono::NaiveDateTime,
}

/// Represents a partial update to a survey
#[typeshare]
#[derive(AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name=surveys)]
pub struct SurveyPatch {
    pub title: Option<String>,
    pub description: Option<String>,
    pub published: Option<bool>,
    pub questions: Option<SurveyQuestions>,
}

#[derive(Insertable)]
#[diesel(table_name=surveys)]
pub struct NewSurvey {
    owner_id: i32,
}

impl NewSurvey {
    pub fn new(owner_id: i32) -> Self {
        Self { owner_id }
    }
}

/// Used to list surveys, like on the page where you can see all your surveys
#[typeshare]
#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name=surveys)]
pub struct ListedSurvey {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub published: bool,
    pub owner_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
#[typeshare(serialized_as = "Vec<SurveyQuestion>")]
pub struct SurveyQuestions(pub Vec<SurveyQuestion>);

impl From<Vec<SurveyQuestion>> for SurveyQuestions {
    fn from(v: Vec<SurveyQuestion>) -> Self {
        Self(v)
    }
}

impl From<SurveyQuestions> for Vec<SurveyQuestion> {
    fn from(v: SurveyQuestions) -> Self {
        v.0
    }
}

impl FromSql<Jsonb, Pg> for SurveyQuestions {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for SurveyQuestions {
    fn to_sql(&self, out: &mut diesel::serialize::Output<Pg>) -> diesel::serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}

#[typeshare]
#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name=responses)]
pub struct SurveyResponse {
    pub survey_id: i32,
    #[typeshare(serialized_as = "String")]
    pub responder_uuid: Uuid,
    pub content: SurveyResponses,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=responses)]
pub struct NewSurveyResponse {
    pub survey_id: i32,
    pub responder_uuid: Uuid,
    pub content: SurveyResponses,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
#[typeshare(serialized_as = "HashMap<String, Response>")]
pub struct SurveyResponses(pub HashMap<Uuid, crate::questions::Response>);

impl From<HashMap<Uuid, crate::questions::Response>> for SurveyResponses {
    fn from(v: HashMap<Uuid, crate::questions::Response>) -> Self {
        Self(v)
    }
}

impl From<SurveyResponses> for HashMap<Uuid, crate::questions::Response> {
    fn from(v: SurveyResponses) -> Self {
        v.0
    }
}

impl FromSql<Jsonb, Pg> for SurveyResponses {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for SurveyResponses {
    fn to_sql(&self, out: &mut diesel::serialize::Output<Pg>) -> diesel::serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}
