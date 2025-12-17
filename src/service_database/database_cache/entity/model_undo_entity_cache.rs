use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_undo_entity_cache")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub model_id: String,
    pub table_name: String,
    pub op_type: String,
    pub old_data: String,
    pub new_data: String,
    pub status: i32, // 0=正常,1=已撤销,2=已重做
    pub operator_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
