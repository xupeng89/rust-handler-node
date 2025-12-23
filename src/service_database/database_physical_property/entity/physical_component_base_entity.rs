use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pp_component_base_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub component_id: i32,

    pub code: String,

    pub name: String,

    pub ref_id: i32,

    pub value: f32,

    pub unit_type: String,

    pub is_default: i32,

    pub is_show: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
