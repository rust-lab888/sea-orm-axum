//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}
