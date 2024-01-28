use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait QuestionDAO {
    async fn create_question(&self, question: QuestionFields) -> Result<Question, DBError>;
    async fn delete_question(&self, question_uuid: Uuid) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<Question>, DBError>;
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
impl QuestionDAO for DAO {
    async fn create_question(&self, question: QuestionFields) -> Result<Question, DBError> {
        let record = sqlx::query!(
            r#"
                INSERT INTO questions (title, description)
                VALUES ($1, $2)
                RETURNING *
            "#,
            question.title,
            question.description
        )
        .fetch_one(&self.database)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        return Ok(Question {
            question_uuid: record.id,
            detail: QuestionFields {
                title: record.title,
                description: record.description,
            },
            created_at: record.created_at,
        });
    }

    async fn delete_question(&self, id: Uuid) -> Result<(), DBError> {
        if id.is_nil() {
            return Err(DBError::InvalidUUID(format!("Invalid question id: {}", id)));
        }

        sqlx::query!("DELETE FROM questions WHERE id = $1", id)
            .execute(&self.database)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        return Ok(());
    }

    async fn get_questions(&self) -> Result<Vec<Question>, DBError> {
        return Ok(sqlx::query!("SELECT * FROM questions")
            .fetch_all(&self.database)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?
            .into_iter()
            .map(|record| Question {
                question_uuid: record.id,
                detail: QuestionFields {
                    title: record.title,
                    description: record.description,
                },
                created_at: record.created_at,
            })
            .collect());
    }
}
