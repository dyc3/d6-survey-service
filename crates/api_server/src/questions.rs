use serde::{Serialize, Deserialize};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type", content="content")]
enum Question {
	Text(QText)
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QText {
	prompt: String,
	description: String,
	multiline: bool,
}