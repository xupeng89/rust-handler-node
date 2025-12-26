use napi_derive::napi;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_system_variable")]
#[napi(object)] // 方便 NAPI 直接返回此结构
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub code: String,
    #[sea_orm(column_type = "Double")]
    pub value: f64,
    pub sort_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
