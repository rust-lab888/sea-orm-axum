use crate::domain::model::character;
use crate::usecase;
use crate::presentation::app;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, State},
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
pub struct CharacterRequestSchema {
    pub name: String,
}

pub async fn create_character(
    state: State<app::State>,
    Json(body): Json<CharacterRequestSchema>,
) -> Result<Json<character::Model>, StatusCode> {
    let character = usecase::character::create(&state.conn, &body.name)
        .await
        .expect("fail");


    Ok(Json(character))
}

pub async fn update_character(
    state: State<app::State>,
    Path(id): Path<i32>,
    Json(body): Json<CharacterRequestSchema>,
) -> Result<Json<character::Model>, StatusCode> {
    let character = usecase::character::update(&state.conn, id, &body.name)
        .await
        .expect("updateに失敗しました。");

    Ok(Json(character))
}


pub async fn delete_character(
    state: State<app::State>,
    Path(id): Path<i32>,
) -> Result<String, StatusCode> {
    usecase::character::delete(&state.conn, id)
        .await
        .expect("deleteに失敗しました。");

    Ok("character deleted".to_string())
}
