use std::error::Error;

use axum::{
    routing::{get, post},
    Router,
};

mod db;
mod transaction;

async fn alive() -> &'static str {
    "Alive"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(alive))
        .route("/transactions", post(transaction::create))
        .route("/transactions", get(transaction::get_all))
        .route("/transactions/:id_transac", get(transaction::get_by_id));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
