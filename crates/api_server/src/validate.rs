use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{questions::{QMultipleChoice, Choice, QRating, QText, SurveyQuestion, Question}, db::models::Survey};

pub trait Validate {
	fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, thiserror::Error)]
pub enum ValidationError {
	#[error("`{field}` is required")]
	Required { field: String },
	#[error("Field `{field}` - {value} is not in range [{min}, {max}]")]
	NotInRange { field: String, value: i32, min: i32, max: i32 },
	#[error("Field `{field}` is not unique")]
	NotUnique { field: String, value: String },
	#[error("Error validating field `{field}`: {inner}")]
	Inner {
		field: String,
		uuid: Uuid,
		inner: Box<Self>,
	},
}

impl Validate for Survey {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		let mut errors = Vec::new();
		if self.title.is_empty() {
			errors.push(ValidationError::Required {
				field: "title".to_string(),
			});
		}
		let mut question_uuids = Vec::new();
		for question in &self.questions.0 {
			let question_errors = question.validate();
			if let Err(mut question_errors) = question_errors {
				for question_error in question_errors.drain(..) {
					errors.push(ValidationError::Inner {
						field: "questions".to_string(),
						uuid: question.uuid,
						inner: Box::new(question_error),
					});
				}
			}
			if question_uuids.contains(&question.uuid) {
				errors.push(ValidationError::Inner {
					field: "questions".to_string(),
					uuid: question.uuid,
					inner: Box::new(ValidationError::NotUnique {
						field: "uuid".to_string(),
						value: question.uuid.to_string(),
					}),
				});
			} else {
				question_uuids.push(question.uuid);
			}
		}
		if errors.is_empty() {
			Ok(())
		} else {
			Err(errors)
		}
	}
}

impl Validate for SurveyQuestion {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		self.question.validate().map_err(|mut e| e.into_iter().map(|v| ValidationError::Inner {
			field: "question".to_string(),
			uuid: self.uuid,
			inner: Box::new(v),
		}).collect())
	}
}

impl Validate for Question {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		match self {
			Question::Text(q) => q.validate(),
			Question::Rating(q) => q.validate(),
			Question::MultipleChoice(q) => q.validate(),
		}
	}
}

impl Validate for QText {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		let mut errors = Vec::new();
		if self.prompt.is_empty() {
			errors.push(ValidationError::Required {
				field: "prompt".to_string(),
			});
		}
		if errors.is_empty() {
			Ok(())
		} else {
			Err(errors)
		}
	}
}

impl Validate for QRating {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		let mut errors = Vec::new();
		if self.prompt.is_empty() {
			errors.push(ValidationError::Required {
				field: "prompt".to_string(),
			});
		}
		if (2..=10).contains(&self.max_rating) {
			errors.push(ValidationError::NotInRange {
				field: "max_rating".to_string(),
				value: self.max_rating.into(),
				min: 2,
				max: 10,
			});
		}
		if errors.is_empty() {
			Ok(())
		} else {
			Err(errors)
		}
	}
}

impl Validate for QMultipleChoice {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		let mut errors = Vec::new();
		if self.prompt.is_empty() {
			errors.push(ValidationError::Required {
				field: "prompt".to_string(),
			});
		}
		if self.choices.is_empty() {
			errors.push(ValidationError::Required {
				field: "choices".to_string(),
			});
		}
		let mut choice_uuids = Vec::new();
		for choice in &self.choices {
			let choice_errors = choice.validate();
			if let Err(mut choice_errors) = choice_errors {
				for choice_error in choice_errors.drain(..) {
					errors.push(ValidationError::Inner {
						field: "choices".to_string(),
						uuid: choice.uuid,
						inner: Box::new(choice_error),
					});
				}
			}
			if choice_uuids.contains(&choice.uuid) {
				errors.push(ValidationError::Inner {
					field: "choices".to_string(),
					uuid: choice.uuid,
					inner: Box::new(ValidationError::NotUnique {
						field: "uuid".to_string(),
						value: choice.uuid.to_string(),
					}),
				});
			} else {
				choice_uuids.push(choice.uuid);
			}
		}
		if errors.is_empty() {
			Ok(())
		} else {
			Err(errors)
		}
	}
}

impl Validate for Choice {
	fn validate(&self) -> Result<(), Vec<ValidationError>> {
		let mut errors = Vec::new();
		if self.text.is_empty() {
			errors.push(ValidationError::Required {
				field: "text".to_string(),
			});
		}
		if errors.is_empty() {
			Ok(())
		} else {
			Err(errors)
		}
	}
}
