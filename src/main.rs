extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{delete, get, post},
    Router,
};
use persistance::{
    answers_dao::{self, AnswerDAO},
    questions_dao::{self, QuestionDAO},
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod handlers;
mod models;
mod persistance;

use handlers::*;

const MAX_CONNECTIONS: u32 = 5;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionDAO + Send + Sync>,
    pub answers_dao: Arc<dyn AnswerDAO + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // Panic if no .env file exists
    dotenvy::dotenv().unwrap();

    // Panic if DATABASE_URL is not set
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    // Panic if the address is already occupied.
    let listener = TcpListener::bind(address).await.unwrap();
    let app = Router::new()
        .route("/question/:id", delete(delete_question))
        .route("/questions", get(read_questions))
        .route("/question", post(create_question))
        .route("/answer/:id", delete(delete_answer))
        .route("/answers/:question_id", get(read_answers))
        .route("/answer", post(create_answer))
        .with_state(AppState {
            questions_dao: Arc::new(questions_dao::DAO::new(pool.clone())),
            answers_dao: Arc::new(answers_dao::DAO::new(pool.clone())),
        });

    info!(
        "Axum Server Running at: http://{:?}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
