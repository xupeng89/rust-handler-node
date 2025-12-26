use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_model", comment = "模版表")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,

    #[sea_orm(column_name = "model_no", default = "")]
    pub model_no: String,

    #[sea_orm(default = "")]
    pub developer: String,

    #[sea_orm(column_name = "model_name")]
    pub model_name: String,

    #[sea_orm(column_name = "standard_temperature")]
    pub standard_temperature: f64,

    #[sea_orm(column_name = "standard_temperature_unit")]
    pub standard_temperature_unit: String,

    #[sea_orm(column_name = "standard_pressure")]
    pub standard_pressure: f64,

    #[sea_orm(column_name = "standard_pressure_unit")]
    pub standard_pressure_unit: String,

    #[sea_orm(column_name = "grid_state")]
    pub grid_state: String,

    #[sea_orm(column_name = "grid_color")]
    pub grid_color: String,

    #[sea_orm(column_name = "grid_size")]
    pub grid_size: i32,

    #[sea_orm(column_name = "create_at")]
    pub create_at: NaiveDateTime,

    #[sea_orm(column_name = "update_at")]
    pub update_at: NaiveDateTime,

    /// 状态, 0:正常，1:删除，2:禁用
    pub status: i32,

    #[sea_orm(column_name = "default_model_id")]
    pub default_model_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
