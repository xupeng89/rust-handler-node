use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum ModelAutoShutterEntity {
    Table,
}

#[derive(Iden)]
enum ModelAutoShutterEntityNew {
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

#[derive(Iden)]
enum ModelAutoShutterEntityOldRollback {
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

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1️⃣ 创建索引
        // manager
        //     .create_index(
        //         Index::create()
        //             .name("idx_auto_shutter_model_id_update_at")
        //             .table(ModelAutoShutterEntity::Table)
        //             .col(ModelAutoShutterEntity::ModelId)
        //             .col(ModelAutoShutterEntity::UpdateAt)
        //             .if_not_exists()
        //             .to_owned(),
        //     )
        //     .await?;

        // 2️⃣ 检查 update_at 类型是否为 TEXT
        let stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            "PRAGMA table_info(model_auto_shutter_entity);".to_string(),
        );
        let pragma_rows = db.query_all_raw(stmt).await?;

        let mut update_at_is_text = false;
        for row in pragma_rows.iter() {
            let name: String = row.try_get::<String>("", "name")?;
            let ty: String = row.try_get::<String>("", "type")?;
            if name.eq_ignore_ascii_case("update_at") && ty.eq_ignore_ascii_case("datetime_text") {
                update_at_is_text = true;
                break;
            }
        }

        // let mut update_at_is_text = false;
        // for row in pragma_rows.iter() {
        //     let name: String = row.try_get("name")?;
        //     let ty: String = row.try_get("type")?;
        //     if name.eq_ignore_ascii_case("update_at") && ty.eq_ignore_ascii_case("TEXT") {
        //         update_at_is_text = true;
        //         break;
        //     }
        // }

        if !update_at_is_text {
            eprintln!("ℹ️ update_at is not TEXT, skip migration");
            return Ok(());
        }

        eprintln!("🔧 Migrating update_at TEXT -> BIGINT");

        // 3️⃣ 创建新表，update_at BIGINT
        manager
            .create_table(
                Table::create()
                    .table(ModelAutoShutterEntityNew::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::SimTime)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::UpdateAt)
                            .big_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(ModelAutoShutterEntityNew::Objects).text())
                    .col(ColumnDef::new(ModelAutoShutterEntityNew::Sysvars).text())
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::ModelId)
                            .string_len(64)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::BaseStateCode)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::Username)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::StateIndex)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityNew::StateDesc)
                            .string_len(128)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 4️⃣ 迁移数据：TEXT -> BIGINT
        db.execute_unprepared(
            r#"
            INSERT INTO model_auto_shutter_entity_new (id, sim_time, update_at, objects, sysvars, model_id, base_state_code, username, state_index, state_desc)
            SELECT
                id,
                sim_time,
                CAST(update_at AS INTEGER) AS update_at,
                objects,
                sysvars,
                model_id,
                base_state_code,
                username,
                state_index,
                state_desc
            FROM model_auto_shutter_entity;
            "#,
        )
        .await?;

        // 5️⃣ 删除旧表
        manager
            .drop_table(
                Table::drop()
                    .table(ModelAutoShutterEntity::Table)
                    .to_owned(),
            )
            .await?;

        // 6️⃣ 重命名新表
        manager
            .rename_table(
                Table::rename()
                    .table(
                        ModelAutoShutterEntityNew::Table,
                        ModelAutoShutterEntity::Table,
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 删除索引
        // manager
        //     .drop_index(
        //         Index::drop()
        //             .name("idx-auto_shutter-model_id-update_at")
        //             .table(ModelAutoShutterEntity::Table)
        //             .to_owned(),
        //     )
        //     .await?;

        // 回滚 BIGINT -> TEXT
        manager
            .create_table(
                Table::create()
                    .table(ModelAutoShutterEntityOldRollback::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::SimTime)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::UpdateAt)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(ModelAutoShutterEntityOldRollback::Objects).text())
                    .col(ColumnDef::new(ModelAutoShutterEntityOldRollback::Sysvars).text())
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::ModelId)
                            .string_len(64)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::BaseStateCode)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::Username)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::StateIndex)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityOldRollback::StateDesc)
                            .string_len(128)
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(
            r#"
            INSERT INTO model_auto_shutter_entity_old_rollback (id, sim_time, update_at, objects, sysvars, model_id, base_state_code, username, state_index, state_desc)
            SELECT
                id,
                sim_time,
                CAST(update_at AS TEXT) AS update_at,
                objects,
                sysvars,
                model_id,
                base_state_code,
                username,
                state_index,
                state_desc
            FROM model_auto_shutter_entity;
            "#,
        )
        .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ModelAutoShutterEntity::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table(
                        ModelAutoShutterEntityOldRollback::Table,
                        ModelAutoShutterEntity::Table,
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
