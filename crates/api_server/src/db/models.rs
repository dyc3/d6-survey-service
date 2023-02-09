#[derive(Queryable)]
pub struct User {
	pub id: u32,
	pub username: String,
	pub password: String,
}

// #[derive(Insertable)]
// #[diesel(table_name="users")]
pub struct NewUser {
	pub username: String,
	pub password: String,
}