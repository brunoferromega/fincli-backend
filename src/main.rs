use std::error::Error;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use mongodb::{
    bson::{doc, Document},
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TransactionRD {
    title: String,
    amount: f32,
    date_time: String,
    description: String,
}

async fn alive() -> &'static str {
    "Alive"
}

async fn save_transaction(
    Json(transaction): Json<TransactionRD>,
) -> (StatusCode, Json<TransactionRD>) {
    dbg!(&transaction);
    (StatusCode::OK, Json(transaction))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(alive))
        .route("/transactions", post(save_transaction))
        .route("/transactions", get(get_transaction));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_transaction() -> StatusCode {
    let uri = "mongodb://127.0.0.1:27017/";
    let client = mongodb::Client::with_uri_str(uri).await.unwrap();
    let database = client.database("financial");
    let collection: Collection<Document> = database.collection("transactions");
    let transaction_records = collection.find_one(doc! { "title": "salary" }).await.unwrap();
    println!("Found transactions:\n{:#?}", transaction_records);
    StatusCode::OK
}
