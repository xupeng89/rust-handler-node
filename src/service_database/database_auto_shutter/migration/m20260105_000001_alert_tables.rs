use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum ModelAutoShutterEntity {
    Table,
    UpdateAt,
    ModelId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1️⃣ 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_auto_shutter_model_id_update_at_desc")
                    .table(ModelAutoShutterEntity::Table)
                    .col(ModelAutoShutterEntity::ModelId)
                    .col((ModelAutoShutterEntity::UpdateAt, IndexOrder::Desc))
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除索引
        manager
            .drop_index(
                Index::drop()
                    .name("idx_auto_shutter_model_id_update_at_desc")
                    .table(ModelAutoShutterEntity::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
