use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =========================================================================
        // model_shutter_entity
        // =========================================================================
        manager
            .create_table(
                Table::create()
                    .table(ModelShutterEntity::Table)
                    .if_not_exists()
                    // 对应 TypeORM 的 id: string @PrimaryColumn
                    .col(
                        ColumnDef::new(ModelShutterEntity::Id)
                            .string_len(36) // TypeORM string id 通常为 UUID/VARCHAR，这里假设长度为 36
                            .not_null()
                            .primary_key(),
                    )
                    // 对应 TypeORM 的 name: string @Column(name: "name", default: "")
                    .col(
                        ColumnDef::new(ModelShutterEntity::Name)
                            .string_len(255)
                            .default(""),
                    )
                    // 对应 TypeORM 的 index: number @Column(name: "index_num")
                    .col(
                        ColumnDef::new(ModelShutterEntity::IndexNum)
                            .integer()
                            .not_null(),
                    )
                    // 对应 TypeORM 的 updateAt: string @Column(name: "update_at")
                    .col(
                        ColumnDef::new(ModelShutterEntity::UpdateAt)
                            .string_len(64) // 假设用于存储时间字符串
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelShutterEntity::ModelId)
                            .string_len(64)
                            .default(""),
                    )
                    // 对应 TypeORM 的 userName: string @Column(name: "username", nullable: true)
                    .col(
                        ColumnDef::new(ModelShutterEntity::Username)
                            .string_len(64)
                            .null(),
                    )
                    // 对应 TypeORM 的 type: number @Column(name: "type", nullable: true)
                    .col(ColumnDef::new(ModelShutterEntity::Type).integer().null())
                    // 对应 TypeORM 的 stateIndex: number @Column(name: "state_index", nullable: true)
                    .col(
                        ColumnDef::new(ModelShutterEntity::StateIndex)
                            .integer()
                            .null(),
                    )
                    // 对应 TypeORM 的 stateDesc: string @Column(name: "state_desc", nullable: true)
                    .col(
                        ColumnDef::new(ModelShutterEntity::StateDesc)
                            .string_len(128)
                            .null(),
                    )
                    // 对应 TypeORM 的 status: string @Column(name: "status", default: "")
                    .col(
                        ColumnDef::new(ModelShutterEntity::BaseStateCode)
                            .string_len(32)
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;
        // 2. 创建数据表 (存大数据，Blob类型)
        manager
            .create_table(
                Table::create()
                    .table(ModelShutterDataEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelShutterDataEntity::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    ) // 与主表ID一一对应
                    .col(ColumnDef::new(ModelShutterDataEntity::Objects).blob()) // 改用 Blob 存压缩数据
                    .col(ColumnDef::new(ModelShutterDataEntity::Sysvars).blob())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_shutter_data_id")
                            .from(ModelShutterDataEntity::Table, ModelShutterDataEntity::Id)
                            .to(ModelShutterEntity::Table, ModelShutterEntity::Id)
                            .on_delete(ForeignKeyAction::Cascade), // 主表删了，数据自动删
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelShutterEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(ModelShutterDataEntity::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

// 对应数据库的表名和列名
#[derive(Iden)]
enum ModelShutterEntity {
    Table,
    Id,
    Name,
    IndexNum,
    UpdateAt,
    ModelId,
    Username,
    Type,
    StateIndex,
    StateDesc,
    BaseStateCode,
}

#[derive(Iden)]
enum ModelShutterDataEntity {
    Table,
    Id,
    Objects,
    Sysvars,
}
