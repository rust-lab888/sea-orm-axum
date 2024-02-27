use crate::domain::model::character;
use sea_orm::{DbErr};

pub trait CharacterRepository {
    async fn get_characters(&self) -> Result<Vec<character::Model>, DbErr>;
    async fn create_character(&self, name: &str) -> Result<character::Model, DbErr>;
}