use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct State {
    pub conn: DatabaseConnection,
}

