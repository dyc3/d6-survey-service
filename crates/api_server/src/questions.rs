use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyQuestion {
    #[typeshare(serialized_as = "String")]
    pub uuid: Uuid,
    pub required: bool,
    pub question: Question,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Question {
    Text(QText),
    Rating(QRating),
    MultipleChoice(QMultipleChoice)
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QText {
    prompt: String,
    description: String,
    multiline: bool,
}

/// Represents a question like "On a scale of 1 to N, how do you feel about X?"
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRating {
    prompt: String,
    description: String,
    max_rating: u8,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QMultipleChoice {
    prompt: String,
    description: String,
}
