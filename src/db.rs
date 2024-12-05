use crate::transaction::Transaction;

const URI_DB: &str = "mongodb://127.0.0.1:27017/";

pub async fn db_transaction() -> mongodb::Collection<Transaction> {
    mongodb::Client::with_uri_str(URI_DB)
        .await
        .unwrap()
        .database("financial_db")
        .collection::<Transaction>("transactions")
}
