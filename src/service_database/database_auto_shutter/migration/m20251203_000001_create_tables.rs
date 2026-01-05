use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =========================================================================
        // 3. model_auto_shutter_entity_cache
        // =========================================================================
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
                        ColumnDef::new(ModelAutoShutterEntity::SimTime)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::UpdateAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ModelAutoShutterEntity::Objects).text())
                    .col(ColumnDef::new(ModelAutoShutterEntity::Sysvars).text())
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::ModelId)
                            .string_len(64)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::BaseStateCode)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntity::Username)
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

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除顺序通常与创建顺序相反

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
    SimTime,
    UpdateAt,
    Objects,
    Sysvars,
    ModelId,
    BaseStateCode,
    Username,
    StateIndex,
    StateDesc,
}
