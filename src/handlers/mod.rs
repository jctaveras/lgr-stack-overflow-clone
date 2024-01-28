pub mod inner;

use crate::{models::*, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use inner::*;
use uuid::Uuid;

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        return match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::InternalError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        };
    }
}

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<QuestionFields>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    return inner::create_question(question, questions_dao.as_ref())
        .await
        .map(Json);
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    return inner::read_questions(questions_dao.as_ref())
        .await
        .map(Json);
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    return inner::delete_question(
        Uuid::parse_str(&id).unwrap_or(Uuid::nil()),
        questions_dao.as_ref(),
    )
    .await
    .map(Json);
}

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<AnswerFields>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    return inner::create_answer(answer, answers_dao.as_ref())
        .await
        .map(Json);
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Path(question_id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    return inner::read_answers(
        Uuid::parse_str(&question_id).unwrap_or(Uuid::nil()),
        answers_dao.as_ref(),
    )
    .await
    .map(Json);
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    return inner::delete_answer(
        Uuid::parse_str(&id).unwrap_or(Uuid::nil()),
        answers_dao.as_ref(),
    )
    .await
    .map(Json);
}
