use diesel::{
    deserialize::FromSql,
    pg::{Pg, PgValue},
    serialize::ToSql,
    sql_types::Jsonb,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::schema::{surveys, users},
    questions::SurveyQuestion,
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

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
pub struct SurveyQuestions(pub Vec<SurveyQuestion>);

impl FromSql<Jsonb, Pg> for SurveyQuestions {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl ToSql<Jsonb, Pg> for SurveyQuestions {
    fn to_sql(&self, out: &mut diesel::serialize::Output<Pg>) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, out)
    }
}
