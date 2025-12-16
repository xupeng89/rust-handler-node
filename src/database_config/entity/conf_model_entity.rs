use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
/// 模型默认配置表
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_model")] // 对应 @Entity({ name: "conf_model" })
pub struct Model {
    /**
     * 主键
     * 对应 TypeORM 的 @PrimaryGeneratedColumn
     */
    #[sea_orm(primary_key)]
    pub id: String,

    /// 公式名称
    #[sea_orm(column_name = "model_no")]
    pub model_no: String,

    #[sea_orm(column_name = "develop_name")]
    pub develop_name: String,

    #[sea_orm(column_name = "model_name")]
    pub model_name: String,

    #[sea_orm(column_name = "standard_temperature")]
    pub standard_temperature: i32,

    #[sea_orm(column_name = "standard_temperature_unit")]
    pub standard_temperature_unit: String,

    #[sea_orm(column_name = "standard_pressure")]
    pub standard_pressure: i32,

    #[sea_orm(column_name = "standard_pressure_unit")]
    pub standard_pressure_unit: String,

    #[sea_orm(column_name = "grid_state")]
    pub grid_state: String,

    #[sea_orm(column_name = "grid_color")]
    pub grid_color: String,

    #[sea_orm(column_name = "grid_size")]
    pub grid_size: String,

    #[sea_orm(column_name = "create_at")]
    pub create_at: NaiveDateTime,

    #[sea_orm(column_name = "update_at")]
    pub update_at: NaiveDateTime,

    #[sea_orm(column_name = "status")]
    pub status: i8,

    #[sea_orm(column_name = "language")]
    pub language: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 为实体模型实现 ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {}
