use crate::domain::model::character;
use crate::domain::repository::character::CharacterRepository;



pub async fn get_all<R>(repo: &R) -> anyhow::Result<Vec<character::Model>>
where
    R: CharacterRepository,
{
    let characters = repo.get_characters().await?;
    Ok(characters)
}

pub async fn create<R>(repo: &R, name: &str) -> anyhow::Result<character::Model>
where
    R: CharacterRepository,
{
    Ok(repo.create_character(name).await?)
}
