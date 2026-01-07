use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_shutter_entity")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub id: String,
    pub name: String,
    #[sea_orm(column_name = "index_num")]
    pub index_num: i32,
    pub update_at: String,
    pub model_id: String,
    pub user_name: Option<String>,
    #[sea_orm(column_name = "type")]
    pub type_num: Option<i32>,
    pub state_index: Option<i32>,
    pub state_desc: Option<String>,

    pub base_state_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::model_shutter_data_entity::Entity")]
    Data,
}

// 实现关联关系，用于 find_also_related 查询
impl Related<super::model_shutter_data_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Data.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
