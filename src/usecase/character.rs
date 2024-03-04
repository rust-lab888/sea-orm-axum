use crate::domain::model::character;
use crate::domain::repository::character::CharacterRepository;



pub async fn get_all<R>(repo: &R) -> anyhow::Result<Vec<character::Model>>
where
    R: CharacterRepository
{
    Ok(repo.get_characters().await?)
}

pub async fn create<R>(repo: &R, name: &str) -> anyhow::Result<character::Model>
where
    R: CharacterRepository
{
    Ok(repo.create_character(name).await?)
}

pub async fn update<R>(repo: &R, id: i32, name: &str) -> anyhow::Result<character::Model>
where
    R: CharacterRepository
{
    Ok(repo.update_character(id, name).await?)
}

pub async fn delete<R>(repo: &R, id: i32) -> anyhow::Result<()>
where
    R: CharacterRepository
{
    repo.delete_character(id).await?;
    Ok(())
}
