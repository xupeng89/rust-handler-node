use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. 创建主表 (轻量级，用于列表和检索)
        manager
            .create_table(
                Table::create()
                    .table(ModelAutoShutterEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::ModelId)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::SimTime)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::UpdateAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::BaseStateCode)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::UserName)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::StateIndex)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::StateDesc)
                            .string_len(128)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. 创建数据表 (存大数据，Blob类型)
        manager
            .create_table(
                Table::create()
                    .table(ModelAutoShutterDataEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelAutoShutterDataEntity::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    ) // 与主表ID一一对应
                    .col(ColumnDef::new(ModelAutoShutterDataEntity::Objects).blob()) // 改用 Blob 存压缩数据
                    .col(ColumnDef::new(ModelAutoShutterDataEntity::Sysvars).blob())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_auto_shutter_data_id")
                            .from(
                                ModelAutoShutterDataEntity::Table,
                                ModelAutoShutterDataEntity::Id,
                            )
                            .to(ModelAutoShutterEntity::Table, ModelAutoShutterEntity::Id)
                            .on_delete(ForeignKeyAction::Cascade), // 主表删了，数据自动删
                    )
                    .to_owned(),
            )
            .await?;

        // 3. 创建复合索引
        manager
            .create_index(
                Index::create()
                    .name("idx_auto_shutter_model_update")
                    .table(ModelAutoShutterEntity::Table)
                    .col(ModelAutoShutterEntity::ModelId)
                    .col((ModelAutoShutterEntity::UpdateAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ModelAutoShutterDataEntity::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(ModelAutoShutterEntity::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum ModelAutoShutterEntity {
    Table,
    Id,
    ModelId,
    SimTime,
    UpdateAt,
    BaseStateCode,
    UserName,
    StateIndex,
    StateDesc,
}

#[derive(Iden)]
enum ModelAutoShutterDataEntity {
    Table,
    Id,
    Objects,
    Sysvars,
}
