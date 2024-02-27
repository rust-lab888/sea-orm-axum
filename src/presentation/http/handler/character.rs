use crate::domain::model::character;
use crate::usecase;
use crate::presentation::app;
use serde::{Serialize, Deserialize};
use axum::{
    extract::State,
    Json,
    http::StatusCode,
};


pub async fn get_characters(
    state: State<app::State>,
) -> Result<Json<Vec<character::Model>>, StatusCode> {
    let characters = usecase::character::get_all(&state.conn)
        .await
        .expect("cannot find characters");

    Ok(Json(characters))
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterCreateSchema {
    pub name: String,
}

pub async fn create_character(
    state: State<app::State>,
    Json(body): Json<CharacterCreateSchema>,
) -> Result<Json<character::Model>, StatusCode> {
    let character = usecase::character::create(&state.conn, &body.name)
        .await
        .expect("fail");


    Ok(Json(character))
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterUpdateSchema {
    pub id: i32,
    pub name: String,
}

pub async fn update_character(
    state: State<app::State>,
    Json(body): Json<CharacterUpdateSchema>,
) -> Result<Json<character::Model>, StatusCode> {
    let character = usecase::character::update(&state.conn, body.id, &body.name)
        .await
        .expect("updateに失敗しました。");

    Ok(Json(character))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterDeleteSchema {
    pub id: i32,
}

pub async fn delete_character(
    state: State<app::State>,
    Json(body): Json<CharacterDeleteSchema>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    usecase::character::delete(&state.conn, body.id)
        .await
        .expect("deleteに失敗しました。");

    Ok(StatusCode::NO_CONTENT)
}
