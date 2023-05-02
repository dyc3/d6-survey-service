use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::models::{SurveyPatch, SurveyQuestions, SurveyResponses},
    questions::{
        Choice, IsEmpty, QMultipleChoice, QRating, QText, Question, RMultipleChoice, RRating,
        RText, Response, SurveyQuestion,
    },
};

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "type", content = "data")]
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
    #[error("`{uuid}` was not found in `{field}`")]
    NotFound {
        field: String,
        #[typeshare(serialized_as = "String")]
        uuid: Uuid,
    },
    #[error("Response for Question `{uuid}` did not match the expected type")]
    MismatchedTypes {
        #[typeshare(serialized_as = "String")]
        uuid: Uuid,
    },
    #[error("Field `{field}` has an invalid value: {message}")]
    BadValue { field: String, message: String },
    #[error("Error validating field `{field}`: {inner}")]
    Inner {
        /// The name of the field that failed validation.
        field: String,
        /// The UUID of the object inside the field that failed validation.
        #[typeshare(serialized_as = "String")]
        uuid: Uuid,
        #[typeshare(serialized_as = "ValidationError")]
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
        self.question.validate()
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

impl Validate for (&SurveyQuestions, &SurveyResponses) {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let (questions, responses) = (&self.0 .0, &self.1 .0);
        let mut errors = Vec::new();

        // Check that all responses have a corresponding question
        let question_uuids = questions.iter().map(|q| q.uuid).collect::<Vec<_>>();
        for q_uuid in responses.keys() {
            if !question_uuids.contains(q_uuid) {
                errors.push(ValidationError::NotFound {
                    field: "question".to_string(),
                    uuid: *q_uuid,
                });
            }
        }

        for question in questions {
            let response = match responses.get(&question.uuid) {
                Some(r) => r,
                None => {
                    // Throw errors for required questions that are missing responses
                    if question.required {
                        errors.push(ValidationError::Inner {
                            field: "question".to_string(),
                            uuid: question.uuid,
                            inner: Box::new(ValidationError::Required {
                                field: "response".to_string(),
                            }),
                        });
                    }
                    continue;
                }
            };

            // Check that all responses are valid
            let result = (question, response).validate();
            if let Err(mut inner_errors) = result {
                for inner_error in inner_errors.drain(..) {
                    errors.push(ValidationError::Inner {
                        field: "question".to_string(),
                        uuid: question.uuid,
                        inner: Box::new(inner_error),
                    });
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

impl Validate for (&SurveyQuestion, &Response) {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let (question, response) = self;
        let mut errors = Vec::new();

        let inner = match (&question.question, &response) {
            (Question::Text(q), Response::Text(r)) => (q, r).validate(),
            (Question::Rating(q), Response::Rating(r)) => (q, r).validate(),
            (Question::MultipleChoice(q), Response::MultipleChoice(r)) => (q, r).validate(),
            _ => Err(vec![ValidationError::MismatchedTypes {
                uuid: question.uuid,
            }]),
        };
        if let Err(e) = inner {
            errors.append(
                &mut e
                    .into_iter()
                    .map(|v| ValidationError::Inner {
                        field: "question".to_string(),
                        uuid: question.uuid,
                        inner: Box::new(v),
                    })
                    .collect(),
            );
        }

        if question.required && response.is_empty() {
            errors.push(ValidationError::Required {
                field: "response".to_string(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for (&QText, &RText) {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let (question, response) = self;
        let mut errors = Vec::new();
        if !question.multiline && response.text.contains('\n') {
            errors.push(ValidationError::BadValue {
                field: "text".to_string(),
                message: "Text must not contain newlines".to_owned(),
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for (&QRating, &RRating) {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let (question, response) = self;
        let mut errors = Vec::new();
        if !(1..=question.max_rating).contains(&response.rating) {
            errors.push(ValidationError::NotInRange {
                field: "rating".to_string(),
                value: response.rating.into(),
                min: 1,
                max: question.max_rating.into(),
            });
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Validate for (&QMultipleChoice, &RMultipleChoice) {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let (question, response) = self;
        let mut errors = Vec::new();
        for choice in &response.selected {
            if !question.choices.iter().any(|c| c.uuid == *choice) {
                errors.push(ValidationError::NotFound {
                    field: "choice".to_string(),
                    uuid: *choice,
                });
            }
        }

        if !question.multiple && response.selected.len() > 1 {
            errors.push(ValidationError::BadValue {
                field: "selected".to_string(),
                message: "Multiple choices not allowed, select only one".to_owned(),
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

    mod surveys {
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
                    min_text: "".to_owned(),
                    max_text: "".to_owned(),
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
                .flat_map(|q| q.validate().unwrap_err())
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
                min_text: "".to_owned(),
                max_text: "".to_owned(),
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
                        uuid,
                        text: "Choice 1".to_owned(),
                    },
                    Choice {
                        uuid,
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
                        uuid,
                        required: false,
                        question: q.clone(),
                    },
                    SurveyQuestion {
                        uuid,
                        required: false,
                        question: q,
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

    mod responses {
        use super::*;

        #[test]
        fn text_response_should_be_required() {
            let q = SurveyQuestion {
                uuid: Uuid::new_v4(),
                required: true,
                question: Question::Text(QText {
                    prompt: "Prompt".to_owned(),
                    description: "".to_owned(),
                    multiline: false,
                }),
            };

            let r = Response::Text(RText {
                text: "".to_owned(),
            });

            let errors = (&q, &r).validate().unwrap_err();
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::Required { field } => {
                        assert_eq!(field, "response");
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 1);
        }

        #[test]
        fn text_should_not_be_multiline() {
            let q = QText {
                prompt: "Prompt".to_owned(),
                description: "".to_owned(),
                multiline: false,
            };

            let r = RText {
                text: "Line 1\nLine 2".to_owned(),
            };

            let errors = (&q, &r).validate().unwrap_err();
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::BadValue { field, .. } => {
                        assert_eq!(field, "text");
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 1);
        }

        #[test]
        fn rating_response_should_be_in_range() {
            let q = QRating {
                prompt: "Prompt".to_owned(),
                description: "".to_owned(),
                max_rating: 5,
                min_text: "".to_owned(),
                max_text: "".to_owned(),
            };

            let r1 = RRating { rating: 0 };
            let r2 = RRating {
                rating: q.max_rating + 1,
            };
            let r3 = RRating {
                rating: q.max_rating,
            };

            let mut errors = (&q, &r1).validate().unwrap_err();
            errors.extend((&q, &r2).validate().unwrap_err());
            assert!((&q, &r3).validate().is_ok());
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::NotInRange {
                        field, min, max, ..
                    } => {
                        assert_eq!(field, "rating");
                        assert_eq!(min, &1);
                        assert_eq!(max, &5);
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 2);
        }

        #[test]
        fn multiple_choice_response_should_be_in_choices() {
            let q = QMultipleChoice {
                prompt: "Prompt".to_owned(),
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
                multiple: true,
            };

            let r1 = RMultipleChoice {
                selected: vec![Uuid::new_v4(), Uuid::new_v4()],
            };
            let r2 = RMultipleChoice {
                selected: vec![q.choices[0].uuid],
            };

            let errors = (&q, &r1).validate().unwrap_err();
            assert!((&q, &r2).validate().is_ok());
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::NotFound { field, .. } => {
                        assert_eq!(field, "choice");
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 2);
        }

        #[test]
        fn multiple_choice_only_select_one() {
            let q = QMultipleChoice {
                prompt: "Prompt".to_owned(),
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
            };

            let r1 = RMultipleChoice {
                selected: vec![q.choices[0].uuid, q.choices[1].uuid],
            };
            let r2 = RMultipleChoice {
                selected: vec![q.choices[0].uuid],
            };

            let errors = (&q, &r1).validate().unwrap_err();
            assert!((&q, &r2).validate().is_ok());
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::BadValue { field, .. } => {
                        assert_eq!(field, "selected");
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 1);
        }

        #[test]
        fn multiple_choice_response_required() {
            let q = SurveyQuestion {
                uuid: Uuid::new_v4(),
                required: true,
                question: Question::MultipleChoice(QMultipleChoice {
                    prompt: "Prompt".to_owned(),
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
                    multiple: true,
                }),
            };

            let r1 = Response::MultipleChoice(RMultipleChoice { selected: vec![] });

            let errors = (&q, &r1).validate().unwrap_err();
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::Required { field } => {
                        assert_eq!(field, "response");
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
        }

        #[test]
        fn all_responses_must_have_question() {
            let qs = SurveyQuestions(vec![SurveyQuestion {
                uuid: Uuid::new_v4(),
                required: false,
                question: Question::Text(QText {
                    prompt: "Prompt".to_owned(),
                    description: "".to_owned(),
                    multiline: false,
                }),
            }]);

            let rs1 = SurveyResponses(
                [(
                    qs.0[0].uuid,
                    Response::Text(RText {
                        text: "Text".to_owned(),
                    }),
                )]
                .into(),
            );

            let rs2 = SurveyResponses(
                [(
                    Uuid::new_v4(),
                    Response::Text(RText {
                        text: "Text".to_owned(),
                    }),
                )]
                .into(),
            );

            assert!((&qs, &rs1).validate().is_ok());
            let errors = (&qs, &rs2).validate().unwrap_err();
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::NotFound { field, .. } => {
                        assert_eq!(field, "question");
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 1);
        }

        #[test]
        fn all_required_questions_must_have_responses() {
            let qs = SurveyQuestions(vec![SurveyQuestion {
                uuid: Uuid::new_v4(),
                required: true,
                question: Question::Text(QText {
                    prompt: "Prompt".to_owned(),
                    description: "".to_owned(),
                    multiline: false,
                }),
            }]);

            let rs1 = SurveyResponses(
                [(
                    qs.0[0].uuid,
                    Response::Text(RText {
                        text: "Text".to_owned(),
                    }),
                )]
                .into(),
            );

            let rs2 = SurveyResponses([].into());

            assert!((&qs, &rs1).validate().is_ok());
            let errors = (&qs, &rs2).validate().unwrap_err();
            for (i, error) in errors.iter().enumerate() {
                match error {
                    ValidationError::Inner { field, uuid, inner } => {
                        assert_eq!(field, "question");
                        assert_eq!(uuid, &qs.0[0].uuid);
                        match inner.as_ref() {
                            ValidationError::Required { field } => {
                                assert_eq!(field, "response");
                            }
                            _ => panic!("Unexpected inner error at {i}: {error:?}"),
                        }
                    }
                    _ => panic!("Unexpected error at {i}: {error:?}"),
                }
            }
            assert_eq!(errors.len(), 1);
        }
    }
}
