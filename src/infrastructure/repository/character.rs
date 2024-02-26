use crate::domain::model::character;
use crate::domain::repository::character::CharacterRepository;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set, ActiveModelTrait};



impl CharacterRepository for DatabaseConnection {
    async fn get_characters(&self) -> Result<Vec<character::Model>, DbErr> {
        character::Entity::find().all(self).await
    }
    async fn create_character(&self, name: &str) -> Result<character::ActiveModel, DbErr> {
        character::ActiveModel{
            name: Set(name.to_string()),
            ..Default::default()
        }
        .save(self)
        .await
    }
}
