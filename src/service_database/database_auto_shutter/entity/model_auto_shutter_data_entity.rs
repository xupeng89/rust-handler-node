use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    table_name = "model_auto_shutter_data_entity",
    comment = "自动快照持久化保持表"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Blob")]
    pub objects: Vec<u8>, // 二进制存储
    #[sea_orm(column_type = "Blob")]
    pub sysvars: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_auto_shutter_entity::Entity",
        from = "Column::Id",
        to = "super::model_auto_shutter_entity::Column::Id"
    )]
    Main,
}
impl Related<super::model_auto_shutter_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Main.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
