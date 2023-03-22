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
    pub multiple: bool,
    pub choices: Vec<Choice>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    #[typeshare(serialized_as = "String")]
    pub uuid: Uuid,
    pub text: String,
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

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum Response {
    Text(RText),
    Rating(RRating),
    MultipleChoice(RMultipleChoice),
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RText {
    pub text: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RRating {
    pub rating: u8,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RMultipleChoice {
    #[typeshare(serialized_as = "Vec<String>")]
    pub selected: Vec<Uuid>,
}

impl From<RText> for Response {
    fn from(r: RText) -> Self {
        Self::Text(r)
    }
}

impl From<RRating> for Response {
    fn from(r: RRating) -> Self {
        Self::Rating(r)
    }
}

impl From<RMultipleChoice> for Response {
    fn from(r: RMultipleChoice) -> Self {
        Self::MultipleChoice(r)
    }
}

pub(crate) trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for Response {
    fn is_empty(&self) -> bool {
        match self {
            Self::Text(r) => r.is_empty(),
            Self::Rating(r) => r.is_empty(),
            Self::MultipleChoice(r) => r.is_empty(),
        }
    }
}

impl IsEmpty for RText {
    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl IsEmpty for RRating {
    fn is_empty(&self) -> bool {
        self.rating == 0
    }
}

impl IsEmpty for RMultipleChoice {
    fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}
