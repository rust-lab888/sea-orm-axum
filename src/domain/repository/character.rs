use crate::domain::model::character;
use sea_orm::{DbErr, DeleteResult};

pub trait CharacterRepository {
    async fn get_characters(&self) -> Result<Vec<character::Model>, DbErr>;
    async fn create_character(&self, name: &str) -> Result<character::Model, DbErr>;
    async fn update_character(&self, id: i32, name: &str) -> Result<character::Model, DbErr>;
    async fn delete_character(&self, id: i32) -> Result<DeleteResult, DbErr>;
}