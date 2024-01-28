use crate::models::*;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn create_question(Json(question): Json<QuestionFields>) -> impl IntoResponse {
    todo!()
}

pub async fn read_questions() -> impl IntoResponse {
    todo!()
}

pub async fn delete_question(Json(id): Json<String>) {
    todo!()
}

pub async fn create_answer(Json(answer): Json<AnswerFields>) -> impl IntoResponse {
    return (StatusCode::OK, Json(Answer::new(answer))).into_response();
}

pub async fn read_answers(Json(question_id): Json<String>) -> impl IntoResponse {
    todo!()
}

pub async fn delete_answer(Json(id): Json<String>) {
    todo!()
}
