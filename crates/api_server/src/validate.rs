use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::models::SurveyPatch,
    questions::{Choice, QMultipleChoice, QRating, QText, Question, SurveyQuestion},
};

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, thiserror::Error)]
pub enum ValidationError {
    #[error("`{field}` is required")]
    Required { field: String },
    #[error("Field `{field}` - {value} is not in range [{min}, {max}]")]
    NotInRange {
        field: String,
        value: i32,
        min: i32,
        max: i32,
    },
    #[error("Field `{field}` is not unique")]
    NotUnique { field: String, value: String },
    #[error("Error validating field `{field}`: {inner}")]
    Inner {
        field: String,
        uuid: Uuid,
        inner: Box<Self>,
    },
}

impl Validate for SurveyPatch {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        if let Some(title) = &self.title {
            if title.is_empty() {
                errors.push(ValidationError::Required {
                    field: "title".to_string(),
                });
            }
        }
        if let Some(questions) = &self.questions {
            let mut question_uuids = Vec::new();
            for question in &questions.0 {
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
        self.question.validate().map_err(|e| {
            e.into_iter()
                .map(|v| ValidationError::Inner {
                    field: "question".to_string(),
                    uuid: self.uuid,
                    inner: Box::new(v),
                })
                .collect()
        })
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
        if !(2..=10).contains(&self.max_rating) {
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

#[cfg(test)]
mod tests {

    use crate::db::models::SurveyQuestions;

    use super::*;

    #[test]
    fn prompts_should_be_required() {
        let qs: Vec<Question> = vec![
            QText {
                prompt: "".to_owned(),
                description: "".to_owned(),
                multiline: false,
            }
            .into(),
            QRating {
                prompt: "".to_owned(),
                description: "".to_owned(),
                max_rating: 5,
            }
            .into(),
            QMultipleChoice {
                prompt: "".to_owned(),
                description: "".to_owned(),
                choices: vec![
                    Choice {
                        uuid: Uuid::new_v4(),
                        text: "Choice 1".to_owned(),
                    },
                    Choice {
                        uuid: Uuid::new_v4(),
                        text: "Choice 2".to_owned(),
                    },
                ],
                multiple: false,
            }
            .into(),
        ];
        let errors = qs
            .iter()
            .map(|q| q.validate().unwrap_err())
            .flatten()
            .collect::<Vec<_>>();
        for (i, error) in errors.iter().enumerate() {
            match error {
                ValidationError::Required { field } => {
                    assert!(field == "prompt");
                }
                _ => panic!("Unexpected error at {i}: {error:?}"),
            }
        }
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn max_rating_should_be_in_range() {
        let q = QRating {
            prompt: "Prompt".to_owned(),
            description: "".to_owned(),
            max_rating: 1,
        };
        let errors = q.validate().unwrap_err();
        for (i, error) in errors.iter().enumerate() {
            match error {
                ValidationError::NotInRange {
                    field,
                    value,
                    min,
                    max,
                } => {
                    assert!(field == "max_rating");
                    assert_eq!(*value, 1);
                    assert_eq!(*min, 2);
                    assert_eq!(*max, 10);
                }
                _ => panic!("Unexpected error at {i}: {error:?}"),
            }
        }
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn choices_should_be_required() {
        let q = QMultipleChoice {
            prompt: "Prompt".to_owned(),
            description: "".to_owned(),
            choices: vec![],
            multiple: false,
        };
        let errors = q.validate().unwrap_err();
        for (i, error) in errors.iter().enumerate() {
            match error {
                ValidationError::Required { field } => {
                    assert!(field == "choices");
                }
                _ => panic!("Unexpected error at {i}: {error:?}"),
            }
        }
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn choice_text_should_be_required() {
        let q = QMultipleChoice {
            prompt: "Prompt".to_owned(),
            description: "".to_owned(),
            choices: vec![
                Choice {
                    uuid: Uuid::new_v4(),
                    text: "".to_owned(),
                },
                Choice {
                    uuid: Uuid::new_v4(),
                    text: "Choice 2".to_owned(),
                },
            ],
            multiple: false,
        };
        let errors = q.validate().unwrap_err();
        for (i, error) in errors.iter().enumerate() {
            match error {
                ValidationError::Inner {
                    field,
                    uuid: _,
                    inner,
                } => {
                    assert!(field == "choices");
                    match inner.as_ref() {
                        ValidationError::Required { field } => {
                            assert!(field == "text");
                        }
                        _ => panic!("Unexpected error at {i}: {error:?}"),
                    }
                }
                _ => panic!("Unexpected error at {i}: {error:?}"),
            }
        }
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn choice_uuids_should_be_unique() {
        let uuid = Uuid::new_v4();
        let q = QMultipleChoice {
            prompt: "Prompt".to_owned(),
            description: "".to_owned(),
            choices: vec![
                Choice {
                    uuid: uuid.clone(),
                    text: "Choice 1".to_owned(),
                },
                Choice {
                    uuid: uuid.clone(),
                    text: "Choice 2".to_owned(),
                },
            ],
            multiple: false,
        };
        let errors = q.validate().unwrap_err();
        for (i, error) in errors.iter().enumerate() {
            match error {
                ValidationError::Inner {
                    field,
                    uuid: _,
                    inner,
                } => {
                    assert!(field == "choices");
                    match inner.as_ref() {
                        ValidationError::NotUnique { field, value } => {
                            assert!(field == "uuid");
                            assert_eq!(value, &uuid.to_string());
                        }
                        _ => panic!("Unexpected error at {i}: {error:?}"),
                    }
                }
                _ => panic!("Unexpected error at {i}: {error:?}"),
            }
        }
    }

    #[test]
    fn questions_should_be_unique() {
        let uuid = Uuid::new_v4();
        let q = Question::Text(QText {
            prompt: "Prompt".to_owned(),
            description: "".to_owned(),
            multiline: false,
        });
        let qs = SurveyPatch {
            questions: Some(SurveyQuestions(vec![
                SurveyQuestion {
                    uuid: uuid.clone(),
                    required: false,
                    question: q.clone(),
                },
                SurveyQuestion {
                    uuid: uuid.clone(),
                    required: false,
                    question: q.clone(),
                },
            ])),
            ..Default::default()
        };
        let errors = qs.validate().unwrap_err();
        for (i, error) in errors.iter().enumerate() {
            match error {
                ValidationError::Inner {
                    field,
                    uuid: _,
                    inner,
                } => {
                    assert!(field == "questions");
                    match inner.as_ref() {
                        ValidationError::NotUnique { field, value } => {
                            assert!(field == "uuid");
                            assert_eq!(value, &uuid.to_string());
                        }
                        _ => panic!("Unexpected error at {i}: {error:?}"),
                    }
                }
                _ => panic!("Unexpected error at {i}: {error:?}"),
            }
        }
    }
}
