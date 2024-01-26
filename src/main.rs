use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post}, Router
};
use tokio::net::TcpListener;

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    // Panic if the address is already occupied.
    let listener = TcpListener::bind(address).await.unwrap();
    let app = Router::new()
        .route("/question", delete(delete_question))
        .route("/questions", get(read_questions))
        .route("/question", post(create_question))
        .route("/answer", delete(delete_answer))
        .route("/answers", get(read_answers))
        .route("/answer", post(create_answer));

    println!("Axum Server Running at: http://{:?}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
