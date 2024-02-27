use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct State {
    pub conn: DatabaseConnection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResult {
    pub message: String,
}

