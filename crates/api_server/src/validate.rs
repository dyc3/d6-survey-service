use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::questions::{QMultipleChoice, Choice};

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
