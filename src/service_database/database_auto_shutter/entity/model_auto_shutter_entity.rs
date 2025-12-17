use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_auto_shutter_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    // 字段名变回驼峰命名，但保留 column_name 映射到数据库的 snake_case
    #[sea_orm(column_name = "sim_time", default = "")]
    pub sim_time: String,

    #[sea_orm(column_name = "update_at", default = "")]
    pub update_at: String,

    // 假设 objects 和 sysvars 存储的是 JSON 字符串
    #[sea_orm(default = "")]
    pub objects: String,

    #[sea_orm(default = "")]
    pub sysvars: String,

    #[sea_orm(column_name = "model_id", default = "")]
    pub model_id: String,

    #[sea_orm(default = "")]
    pub base_state_code: String,

    #[sea_orm(column_name = "username")]
    pub user_name: Option<String>,

    #[sea_orm(column_name = "state_index")]
    pub state_index: Option<i32>,

    #[sea_orm(column_name = "state_desc")]
    pub state_desc: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
