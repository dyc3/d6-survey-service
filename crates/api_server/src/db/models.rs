use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
	pub id: u32,
	pub username: String,
	pub password: String,
}