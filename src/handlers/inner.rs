use uuid::Uuid;

use crate::persistance::{answers_dao::AnswerDAO, questions_dao::QuestionDAO};

use super::{Answer, AnswerFields, DBError, Question, QuestionFields};

#[derive(Debug, PartialEq)]
pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

impl HandlerError {
    fn default_internal_error() -> Self {
        return Self::InternalError(String::from("Something went wrong! Please try again."));
    }
}

pub async fn create_question(
    question: QuestionFields,
    dao: &(dyn QuestionDAO + Send + Sync),
) -> Result<Question, HandlerError> {
    return Ok(dao
        .create_question(question)
        .await
        .map_err(|_| HandlerError::default_internal_error())?);
}

pub async fn read_questions(
    dao: &(dyn QuestionDAO + Send + Sync),
) -> Result<Vec<Question>, HandlerError> {
    return Ok(dao
        .get_questions()
        .await
        .map_err(|_| HandlerError::default_internal_error())?);
}

pub async fn delete_question(
    id: Uuid,
    dao: &(dyn QuestionDAO + Send + Sync),
) -> Result<(), HandlerError> {
    return Ok(dao.delete_question(id).await.map_err(|e| {
        return match e {
            DBError::InvalidUUID(message) => HandlerError::BadRequest(message),
            DBError::Other(_) => HandlerError::default_internal_error(),
        };
    })?);
}

pub async fn create_answer(
    answer: AnswerFields,
    dao: &(dyn AnswerDAO + Send + Sync),
) -> Result<Answer, HandlerError> {
    return Ok(dao.create_answer(answer).await.map_err(|e| {
        return match e {
            DBError::InvalidUUID(message) => HandlerError::BadRequest(message),
            DBError::Other(_) => HandlerError::default_internal_error(),
        };
    })?);
}

pub async fn read_answers(
    question_id: Uuid,
    dao: &(dyn AnswerDAO + Send + Sync),
) -> Result<Vec<Answer>, HandlerError> {
    return Ok(dao.get_answers(question_id).await.map_err(|e| {
        return match e {
            DBError::InvalidUUID(message) => HandlerError::BadRequest(message),
            DBError::Other(_) => HandlerError::default_internal_error(),
        };
    })?);
}

pub async fn delete_answer(
    id: Uuid,
    dao: &(dyn AnswerDAO + Send + Sync),
) -> Result<(), HandlerError> {
    return Ok(dao.delete_answer(id).await.map_err(|e| {
        return match e {
            DBError::InvalidUUID(message) => HandlerError::BadRequest(message),
            DBError::Other(_) => HandlerError::default_internal_error(),
        };
    })?);
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use sqlx::Error;
    use tokio::sync::Mutex;

    struct QuestionsDaoMock {
        create_question_response: Mutex<Option<Result<Question, DBError>>>,
        delete_question_response: Mutex<Option<Result<(), DBError>>>,
        get_questions_response: Mutex<Option<Result<Vec<Question>, DBError>>>,
    }

    impl QuestionsDaoMock {
        pub fn new() -> Self {
            QuestionsDaoMock {
                create_question_response: Mutex::new(None),
                delete_question_response: Mutex::new(None),
                get_questions_response: Mutex::new(None),
            }
        }
        pub fn mock_create_question(&mut self, response: Result<Question, DBError>) {
            self.create_question_response = Mutex::new(Some(response));
        }
        pub fn mock_delete_question(&mut self, response: Result<(), DBError>) {
            self.delete_question_response = Mutex::new(Some(response));
        }
        pub fn mock_get_questions(&mut self, response: Result<Vec<Question>, DBError>) {
            self.get_questions_response = Mutex::new(Some(response));
        }
    }

    #[async_trait]
    impl QuestionDAO for QuestionsDaoMock {
        async fn create_question(&self, _: QuestionFields) -> Result<Question, DBError> {
            self.create_question_response
                .lock()
                .await
                .take()
                .expect("create_question_response should not be None.")
        }
        async fn delete_question(&self, _: Uuid) -> Result<(), DBError> {
            self.delete_question_response
                .lock()
                .await
                .take()
                .expect("delete_question_response should not be None.")
        }
        async fn get_questions(&self) -> Result<Vec<Question>, DBError> {
            self.get_questions_response
                .lock()
                .await
                .take()
                .expect("get_questions_response should not be None.")
        }
    }

    struct AnswersDaoMock {
        create_answer_response: Mutex<Option<Result<Answer, DBError>>>,
        delete_answer_response: Mutex<Option<Result<(), DBError>>>,
        get_answers_response: Mutex<Option<Result<Vec<Answer>, DBError>>>,
    }

    impl AnswersDaoMock {
        pub fn new() -> Self {
            AnswersDaoMock {
                create_answer_response: Mutex::new(None),
                delete_answer_response: Mutex::new(None),
                get_answers_response: Mutex::new(None),
            }
        }
        pub fn mock_create_answer(&mut self, response: Result<Answer, DBError>) {
            self.create_answer_response = Mutex::new(Some(response));
        }
        pub fn mock_delete_answer(&mut self, response: Result<(), DBError>) {
            self.delete_answer_response = Mutex::new(Some(response));
        }
        pub fn mock_get_answers(&mut self, response: Result<Vec<Answer>, DBError>) {
            self.get_answers_response = Mutex::new(Some(response));
        }
    }

    #[async_trait]
    impl AnswerDAO for AnswersDaoMock {
        async fn create_answer(&self, _: AnswerFields) -> Result<Answer, DBError> {
            self.create_answer_response
                .lock()
                .await
                .take()
                .expect("create_answer_response should not be None.")
        }
        async fn delete_answer(&self, _: Uuid) -> Result<(), DBError> {
            self.delete_answer_response
                .lock()
                .await
                .take()
                .expect("delete_answer_response should not be None.")
        }
        async fn get_answers(&self, _: Uuid) -> Result<Vec<Answer>, DBError> {
            self.get_answers_response
                .lock()
                .await
                .take()
                .expect("get_answers_response should not be None.")
        }
    }

    #[tokio::test]
    async fn create_question_should_return_question() {
        let question = QuestionFields {
            title: "test title".to_owned(),
            description: "test description".to_owned(),
        };

        let question_detail = Question {
            question_uuid: Uuid::new_v4(),
            detail: QuestionFields {
                title: question.title.clone(),
                description: question.description.clone(),
            },
            created_at: chrono::offset::Utc::now(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_create_question(Ok(question_detail.clone()));

        let questions_dao: Box<dyn QuestionDAO + Send + Sync> = Box::new(questions_dao);

        let result = create_question(question, questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), question_detail);
    }

    #[tokio::test]
    async fn create_question_should_return_error() {
        let question = QuestionFields {
            title: "test title".to_owned(),
            description: "test description".to_owned(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_create_question(Err(DBError::InvalidUUID("test".to_owned())));

        let questions_dao: Box<dyn QuestionDAO + Send + Sync> = Box::new(questions_dao);

        let result = create_question(question, questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn read_questions_should_return_questions() {
        let question_detail = Question {
            question_uuid: Uuid::new_v4(),
            detail: QuestionFields {
                title: "test title".to_owned(),
                description: "test description".to_owned(),
            },
            created_at: chrono::offset::Utc::now(),
        };

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_get_questions(Ok(vec![question_detail.clone()]));

        let questions_dao: Box<dyn QuestionDAO + Send + Sync> = Box::new(questions_dao);

        let result = read_questions(questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![question_detail]);
    }

    #[tokio::test]
    async fn read_questions_should_return_error() {
        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_get_questions(Err(DBError::InvalidUUID("test".to_owned())));

        let questions_dao: Box<dyn QuestionDAO + Send + Sync> = Box::new(questions_dao);

        let result = read_questions(questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn delete_question_should_succeed() {
        let question_id = Uuid::new_v4();

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_delete_question(Ok(()));

        let questions_dao: Box<dyn QuestionDAO + Send + Sync> = Box::new(questions_dao);

        let result = delete_question(question_id, questions_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn delete_question_should_return_error() {
        let question_id = Uuid::nil();

        let mut questions_dao = QuestionsDaoMock::new();

        questions_dao.mock_delete_question(Err(DBError::Other(Box::new(Error::PoolTimedOut))));

        let questions_dao: Box<dyn QuestionDAO + Send + Sync> = Box::new(questions_dao);

        let result = delete_question(question_id, questions_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_answer() {
        let answer = AnswerFields {
            question_uuid: Uuid::new_v4(),
            content: "test content".to_owned(),
        };

        let answer_detail = Answer {
            answer_uuid: Uuid::new_v4(),
            detail: AnswerFields {
                question_uuid: answer.question_uuid.clone(),
                content: answer.content.clone(),
            },
            created_at: chrono::offset::Utc::now(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_create_answer(Ok(answer_detail.clone()));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), answer_detail);
    }

    #[tokio::test]
    async fn create_answer_should_return_bad_request_error() {
        let answer = AnswerFields {
            question_uuid: Uuid::new_v4(),
            content: "test content".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_create_answer(Err(DBError::InvalidUUID("test".to_owned())));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::BadRequest("".to_owned()))
        );
    }

    #[tokio::test]
    async fn create_answer_should_return_internal_error() {
        let answer = AnswerFields {
            question_uuid: Uuid::new_v4(),
            content: "test content".to_owned(),
        };

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_create_answer(Err(DBError::Other(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "oh no!",
        )))));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = create_answer(answer, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn read_answers_should_return_answers() {
        let answer_detail = Answer {
            answer_uuid: Uuid::new_v4(),
            detail: AnswerFields {
                question_uuid: Uuid::new_v4(),
                content: "test content".to_owned(),
            },
            created_at: chrono::offset::Utc::now(),
        };

        let question_id = Uuid::new_v4();

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_get_answers(Ok(vec![answer_detail.clone()]));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = read_answers(question_id, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![answer_detail]);
    }

    #[tokio::test]
    async fn read_answers_should_return_error() {
        let question_id = Uuid::new_v4();

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_get_answers(Err(DBError::Other(Box::new(Error::WorkerCrashed))));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = read_answers(question_id, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }

    #[tokio::test]
    async fn delete_answer_should_succeed() {
        let answer_id = Uuid::new_v4();

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_delete_answer(Ok(()));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = delete_answer(answer_id, answers_dao.as_ref()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[tokio::test]
    async fn delete_answer_should_return_error() {
        let answer_id = Uuid::new_v4();

        let mut answers_dao = AnswersDaoMock::new();

        answers_dao.mock_delete_answer(Err(DBError::Other(Box::new(Error::PoolClosed))));

        let answers_dao: Box<dyn AnswerDAO + Send + Sync> = Box::new(answers_dao);

        let result = delete_answer(answer_id, answers_dao.as_ref()).await;

        assert!(result.is_err());
        assert!(
            std::mem::discriminant(&result.unwrap_err())
                == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
        );
    }
}
