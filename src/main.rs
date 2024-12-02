use std::error::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use futures::TryStreamExt;
use mongodb::{bson::doc, Collection};

use serde::{Deserialize, Serialize};

const URI_DB: &str = "mongodb://127.0.0.1:27017/";

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    title: String,
    amount: f32,
    date_time: String,
    description: Option<String>,
}

async fn alive() -> &'static str {
    "Alive"
}

async fn save_transaction(Json(transaction): Json<Transaction>) -> Response {
    dbg!(&transaction);
    let coll_client = db_transaction().await;
    let resp_db = coll_client.insert_one(&transaction).await;
    match resp_db {
        Ok(_) => (StatusCode::OK, Json(transaction)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn get_transaction() -> StatusCode {
    let coll_client: Collection<Transaction> = db_transaction().await;
    let mut resp_db = coll_client
        .find(doc! {})
        .await
        .unwrap();

    while let Some(record) = resp_db.try_next().await.unwrap() {
        println!("{:#?}", record);
    }

    StatusCode::OK
}

async fn db_transaction() -> mongodb::Collection<Transaction> {
    mongodb::Client::with_uri_str(URI_DB)
        .await
        .unwrap()
        .database("financial_db")
        .collection::<Transaction>("transactions")
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

    todo!("Implement return a array of transactions and change name for get_all");
}
