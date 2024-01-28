use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct QuestionFields {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Question {
    pub question_uuid: Uuid,
    pub detail: QuestionFields,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnswerFields {
    pub question_uuid: Uuid,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Answer {
    pub answer_uuid: Uuid,
    pub detail: AnswerFields,
    pub created_at: DateTime<Utc>,
}

impl Answer {
    pub fn new(detail: AnswerFields) -> Self {
        Self {
            answer_uuid: Uuid::new_v4(),
            detail,
            created_at: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
