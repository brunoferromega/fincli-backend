use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(alive))
        .route("/transactions", post(save_transaction));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn alive() -> &'static str {
    "Alive"
}

async fn save_transaction(Json(transaction): Json<TransactionRD>) -> (StatusCode, Json<TransactionRD>) {
    dbg!(&transaction);
    (StatusCode::OK, Json(transaction))
}

#[derive(Serialize, Deserialize, Debug)]
struct TransactionRD {
    title: String,
    amount: f32,
    description: Option<String>,
}
