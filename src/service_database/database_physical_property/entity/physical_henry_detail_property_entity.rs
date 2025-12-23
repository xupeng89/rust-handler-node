use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pp_henry_detail_property_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "component_i_id")]
    pub component_i_id: i32,

    #[sea_orm(column_name = "component_i")]
    pub component_i: String,

    #[sea_orm(column_name = "component_j_id")]
    pub component_j_id: i32,

    #[sea_orm(column_name = "component_j")]
    pub component_j: String,

    #[sea_orm(column_name = "source_name")]
    pub source_name: String,

    #[sea_orm(column_name = "aij")]
    pub aij: f64,
    #[sea_orm(column_name = "bij")]
    pub bij: f64,
    #[sea_orm(column_name = "cij")]
    pub cij: f64,
    #[sea_orm(column_name = "dij")]
    pub dij: f64,
    #[sea_orm(column_name = "eij")]
    pub eij: f64,

    #[sea_orm(column_name = "t_lower")]
    pub t_lower: f64,
    #[sea_orm(column_name = "t_upper")]
    pub t_upper: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
