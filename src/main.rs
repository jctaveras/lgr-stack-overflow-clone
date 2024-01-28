extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod handlers;
mod models;
mod persistance;

use handlers::*;

const MAX_CONNECTIONS: u32 = 5;

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
        .unwrap();
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    // Panic if the address is already occupied.
    let listener = TcpListener::bind(address).await.unwrap();
    let app = Router::new()
        .route("/question", delete(delete_question))
        .route("/questions", get(read_questions))
        .route("/question", post(create_question))
        .route("/answer", delete(delete_answer))
        .route("/answers", get(read_answers))
        .route("/answer", post(create_answer));

    info!("********* Question Records *********");

    println!(
        "Axum Server Running at: http://{:?}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
