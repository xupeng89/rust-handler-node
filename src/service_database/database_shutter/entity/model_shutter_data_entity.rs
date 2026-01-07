use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "model_shutter_data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String, // 与主表 ID 对应
    pub objects: Vec<u8>, // 存储 zstd 压缩后的二进制
    pub sysvars: Vec<u8>, // 存储 zstd 压缩后的二进制
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::model_shutter_entity::Entity",
        from = "Column::Id",
        to = "super::model_shutter_entity::Column::Id"
    )]
    Main,
}

impl Related<super::model_shutter_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Main.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
