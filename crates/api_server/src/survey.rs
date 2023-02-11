use serde::{Serialize, Deserialize};

use crate::questions::Question;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Survey {
	pub id: String,
	pub name: String,
	pub description: String,
	pub questions: Vec<Question>,
}