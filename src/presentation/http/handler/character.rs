use crate::domain::model::character;
use crate::usecase;
use crate::presentation::app;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{State},
    response::IntoResponse,
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
struct CharacterCreateSchema {
    name: String,
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
