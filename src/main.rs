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

#[derive(Serialize, Deserialize, Debug)]
struct Transactions {
    transactions: Vec<Transaction>,
}

impl From<Vec<Transaction>> for Transactions {
    fn from(transactions: Vec<Transaction>) -> Transactions {
        Transactions { transactions }
    }
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

async fn get_all_transactions() -> Response {
    let coll_client: Collection<Transaction> = db_transaction().await;
    let resp_db = coll_client
        .find(doc! {})
        .await
        .unwrap();

    let transactions: Vec<Transaction> = resp_db.try_collect().await.unwrap();
    dbg!(&transactions);
    
    let resp_all_transc: Transactions = transactions.into();

    (StatusCode::OK, Json(resp_all_transc)).into_response()
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
        .route("/transactions", get(get_all_transactions));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
