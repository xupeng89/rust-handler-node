use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

// ✅ 为 Relation 实现 RelationTrait（即使没有外键）
impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    panic!("No RelationDef for this Entity") // 或者 return RelationDef::new(...)，如果以后添加外键关系
  }
}

impl ActiveModelBehavior for ActiveModel {}
