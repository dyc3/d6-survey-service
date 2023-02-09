use diesel::prelude::*;

pub struct User {
	id: u32,
	username: String,
	password: String,
}