use serde::{Deserialize, Serialize};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
enum Question {
    Text(QText),
    Rating(QRating),
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QText {
    prompt: String,
    description: String,
    multiline: bool,
}

/// Represents a question like "On a scale of 1 to N, how do you feel about X?"
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct QRating {
    prompt: String,
    description: String,
    max_rating: u8,
}
