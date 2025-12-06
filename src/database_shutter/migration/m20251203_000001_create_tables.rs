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
                    // 对应 TypeORM 的 objects: string @Column(name: "objects", type: "text")
                    .col(ColumnDef::new(ModelShutterEntity::Objects).text())
                    // 对应 TypeORM 的 sysvars: string @Column(name: "sysvars", type: "text")
                    .col(ColumnDef::new(ModelShutterEntity::Sysvars).text())
                    // 对应 TypeORM 的 modelId: string @Column(name: "model_id", default: "")
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
                        ColumnDef::new(ModelShutterEntity::Status)
                            .string_len(32)
                            .default(""),
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
    Objects,
    Sysvars,
    ModelId,
    Username,
    Type,
    StateIndex,
    StateDesc,
    Status,
}
