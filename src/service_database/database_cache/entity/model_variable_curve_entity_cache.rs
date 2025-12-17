use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_variable_curve_entity_cache")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(name = "sim_time")]
    pub sim_time: String,

    #[sea_orm(column_type = "Text")]
    pub datasets: String,

    #[sea_orm(name = "create_at")]
    pub create_at: String,

    #[sea_orm(name = "model_id")]
    pub model_id: String,

    #[sea_orm(name = "config_id")]
    pub config_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
