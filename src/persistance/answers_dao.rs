use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait AnswerDAO {
    async fn create_answer(&self, details: AnswerFields) -> Result<Answer, DBError>;
    async fn delete_answer(&self, id: Uuid) -> Result<(), DBError>;
    async fn get_answers(&self, question_id: Uuid) -> Result<Vec<Answer>, DBError>;
}

pub struct DAO {
    database: PgPool,
}

impl DAO {
    pub fn new(database: PgPool) -> Self {
        return Self { database };
    }
}

#[async_trait]
impl AnswerDAO for DAO {
    async fn create_answer(&self, details: AnswerFields) -> Result<Answer, DBError> {
        if details.question_uuid.is_nil() {
            return Err(DBError::InvalidUUID(format!(
                "Invalid question_id: {}",
                details.question_uuid
            )));
        }

        let record = sqlx::query!(
            r#"
            INSERT INTO answers (content, question_id)
            VALUES ($1, $2)
            RETURNING *;
        "#,
            details.content,
            details.question_uuid
        )
        .fetch_one(&self.database)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        return Ok(Answer {
            answer_uuid: record.id,
            detail: AnswerFields {
                question_uuid: record.question_id,
                content: record.content,
            },
            created_at: record.created_at,
        });
    }

    async fn delete_answer(&self, id: Uuid) -> Result<(), DBError> {
        if id.is_nil() {
            return Err(DBError::InvalidUUID(format!(
                "Invalid answer id provided: {}",
                id
            )));
        }

        sqlx::query!("DELETE FROM answers WHERE id = $1", id)
            .execute(&self.database)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        return Ok(());
    }

    async fn get_answers(&self, question_id: Uuid) -> Result<Vec<Answer>, DBError> {
        if question_id.is_nil() {
            return Err(DBError::InvalidUUID(format!(
                "Invalid question_id: {}",
                question_id
            )));
        }

        let records = sqlx::query!("SELECT * FROM answers WHERE question_id = $1;", question_id)
            .fetch_all(&self.database)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        return Ok(records
            .into_iter()
            .map(|record| Answer {
                answer_uuid: record.id,
                detail: AnswerFields {
                    content: record.content,
                    question_uuid: record.question_id,
                },
                created_at: record.created_at,
            })
            .collect());
    }
}
