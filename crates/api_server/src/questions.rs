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
    MultipleChoice(QMultipleChoice),
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QText {
    pub prompt: String,
    pub description: String,
    pub multiline: bool,
}

/// Represents a question like "On a scale of 1 to N, how do you feel about X?"
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRating {
    pub prompt: String,
    pub description: String,
    pub max_rating: u8,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QMultipleChoice {
    pub prompt: String,
    pub description: String,
}

impl From<QText> for Question {
    fn from(q: QText) -> Self {
        Self::Text(q)
    }
}

impl From<QRating> for Question {
    fn from(q: QRating) -> Self {
        Self::Rating(q)
    }
}

impl From<QMultipleChoice> for Question {
    fn from(q: QMultipleChoice) -> Self {
        Self::MultipleChoice(q)
    }
}
