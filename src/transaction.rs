use std::str::FromStr;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use futures::TryStreamExt;

use mongodb::{
    bson::{self, doc},
    Collection,
};

use serde::{Deserialize, Serialize};

use crate::db::db_transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    title: String,
    amount: f32,
    date_time: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    transactions: Vec<Transaction>,
}

impl From<Vec<Transaction>> for Transactions {
    fn from(transactions: Vec<Transaction>) -> Transactions {
        Transactions { transactions }
    }
}

pub async fn create(Json(transaction): Json<Transaction>) -> Response {
    dbg!(&transaction);
    let coll_client = db_transaction().await;
    let resp_db = coll_client.insert_one(&transaction).await;
    match resp_db {
        Ok(_) => (StatusCode::OK, Json(transaction)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn get_all() -> Response {
    let coll_client: Collection<Transaction> = db_transaction().await;
    let resp_db = coll_client.find(doc! {}).await.unwrap();

    let transactions: Vec<Transaction> = resp_db.try_collect().await.unwrap();
    dbg!(&transactions);

    let resp_all_transc: Transactions = transactions.into();

    (StatusCode::OK, Json(resp_all_transc)).into_response()
}

pub async fn get_by_id(Path(id_transac): Path<String>) -> Response {
    let obj_id = bson::oid::ObjectId::from_str(&id_transac).unwrap();

    let coll_client: Collection<Transaction> = db_transaction().await;
    let resp_db: Option<Transaction> = coll_client.find_one(doc! { "_id": obj_id }).await.unwrap();

    match resp_db {
        Some(transac) => (StatusCode::OK, Json(transac)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}
