use crate::db::schema::users;

#[derive(Queryable)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
	pub username: String,
	pub password: String,
}