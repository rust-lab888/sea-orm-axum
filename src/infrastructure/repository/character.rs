use crate::domain::model::character;
use crate::domain::repository::character::CharacterRepository;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set, ActiveModelTrait, DeleteResult, ModelTrait};


impl CharacterRepository for DatabaseConnection {
    async fn get_characters(&self) -> Result<Vec<character::Model>, DbErr> {
        character::Entity::find().all(self).await
    }
    async fn create_character(&self, name: &str) -> Result<character::Model, DbErr> {
        let character = character::ActiveModel{
            name: Set(name.to_string()),
            ..Default::default()
        };

        character.insert(self).await
    }
    async fn update_character(&self, id: i32, name: &str) -> Result<character::Model, DbErr> {
        let character = character::Entity::find_by_id(id).one(self).await?;
        let mut character: character::ActiveModel = character.unwrap().into();

        character.name = Set(name.to_string());

        character.update(self).await
    }
    async fn delete_character(&self, id: i32) -> Result<DeleteResult, DbErr> {
        let character = character::Entity::find_by_id(id).one(self).await?;

        let character: character::Model = character.unwrap();
        character.delete(self).await
    }
}
