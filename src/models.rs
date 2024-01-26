use chrono::{self, DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemId(Uuid);

impl PartialEq for ItemId {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct QuestionFields {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Question {
    pub question_uuid: ItemId,
    pub detail: QuestionFields,
    pub created_at: DateTime<Local>,
}

impl Question {
    pub fn new(detail: QuestionFields) -> Self {
        Self {
            question_uuid: ItemId(Uuid::new_v4()),
            detail,
            created_at: chrono::offset::Local::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnswerFields {
    pub question_uuid: ItemId,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Answer {
    pub answer_uuid: ItemId,
    pub detail: AnswerFields,
    pub created_at: DateTime<Local>,
}

impl Answer {
    pub fn new(detail: AnswerFields) -> Self {
        Self {
            answer_uuid: ItemId(Uuid::new_v4()),
            detail,
            created_at: chrono::offset::Local::now(),
        }
    }
}
