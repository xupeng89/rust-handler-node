use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // =========================================================================
        // 1. model_undo_entity_cache
        // =========================================================================
        manager
            .create_table(
                Table::create()
                    .table(ModelUndoEntityCache::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelUndoEntityCache::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModelUndoEntityCache::ModelId)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelUndoEntityCache::TableName)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelUndoEntityCache::OpType)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ModelUndoEntityCache::OldData).text())
                    .col(ColumnDef::new(ModelUndoEntityCache::NewData).text())
                    .col(
                        ColumnDef::new(ModelUndoEntityCache::Status)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ModelUndoEntityCache::OperatorAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================================================================
        // 2. model_position_information_entity
        // =========================================================================
        manager
            .create_table(
                Table::create()
                    .table(ModelPositionInformationEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelPositionInformationEntity::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModelPositionInformationEntity::Name)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelPositionInformationEntity::NameDisplay)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModelPositionInformationEntity::TypeNum)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ModelPositionInformationEntity::TypeName)
                            .string_len(64)
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // =========================================================================
        // 3. model_auto_shutter_entity_cache
        // =========================================================================
        manager
            .create_table(
                Table::create()
                    .table(ModelAutoShutterEntityCache::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityCache::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityCache::SimTime)
                            .string_len(32)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityCache::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ModelAutoShutterEntityCache::Objects).text())
                    .col(ColumnDef::new(ModelAutoShutterEntityCache::Sysvars).text())
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityCache::ModelId)
                            .string_len(64)
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelAutoShutterEntityCache::Status)
                            .string_len(32)
                            .default(""),
                    )
                    .col(ColumnDef::new(ModelAutoShutterEntityCache::UserName).string_len(64))
                    .col(ColumnDef::new(ModelAutoShutterEntityCache::StateIndex).integer())
                    .col(ColumnDef::new(ModelAutoShutterEntityCache::StateDesc).string_len(128))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-auto_shutter-model_id-update_at")
                    .table(ModelAutoShutterEntityCache::Table)
                    // 顺序非常重要：先过滤字段，后排序字段
                    .col(ModelAutoShutterEntityCache::ModelId)
                    .col(ModelAutoShutterEntityCache::UpdateAt)
                    .to_owned(),
            )
            .await?;
        // =========================================================================
        // 4. model_variable_curve_entity_cache (新增)
        // =========================================================================
        manager
            .create_table(
                Table::create()
                    .table(ModelVariableCurveEntityCache::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModelVariableCurveEntityCache::Id)
                            .integer()
                            .not_null()
                            .auto_increment() // 通常 Entity ID 是自增的，如果不是请去掉
                            .primary_key(),
                    )
                    // type: "varchar", default: "" -> string() 默认长度通常够用，也可指定 string_len(255)
                    .col(
                        ColumnDef::new(ModelVariableCurveEntityCache::SimTime)
                            .string()
                            .default(""),
                    )
                    // type: "text", default: "" -> text()
                    .col(ColumnDef::new(ModelVariableCurveEntityCache::Datasets).text()) // Text 类型通常不设 default("")，视数据库而定，这里保持简单
                    // name: "ceate_at" -> 对应枚举 CeateAt (自动转蛇形)
                    .col(
                        ColumnDef::new(ModelVariableCurveEntityCache::CreateAt)
                            .string()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelVariableCurveEntityCache::ModelId)
                            .string()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(ModelVariableCurveEntityCache::ConfigId)
                            .string()
                            .default(""),
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
                    .table(ModelVariableCurveEntityCache::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(ModelAutoShutterEntityCache::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(ModelPositionInformationEntity::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ModelUndoEntityCache::Table).to_owned())
            .await?;
        // 1. 删除 model_id 和 update_at 上的联合索引
        manager
            .drop_index(
                Index::drop()
                    .name("idx-auto_shutter-model_id-update_at")
                    .table(ModelAutoShutterEntityCache::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

// --------------------------------------------------------
// IDEN Definitions
// --------------------------------------------------------

#[derive(Iden)]
enum ModelUndoEntityCache {
    Table,
    Id,
    ModelId,
    TableName,
    OpType,
    OldData,
    NewData,
    Status,
    OperatorAt,
}

#[derive(Iden)]
enum ModelPositionInformationEntity {
    Table,
    Id,
    Name,
    NameDisplay,
    TypeNum,
    TypeName,
}

#[derive(Iden)]
enum ModelAutoShutterEntityCache {
    Table,
    Id,
    SimTime,
    UpdateAt,
    Objects,
    Sysvars,
    ModelId,
    Status,
    UserName,
    StateIndex,
    StateDesc,
}

#[derive(Iden)]
enum ModelVariableCurveEntityCache {
    Table,    // 映射为 model_variable_curve_entity_cache
    Id,       // id
    SimTime,  // sim_time
    Datasets, // datasets
    CreateAt, // create_at
    ModelId,  // model_id
    ConfigId, // config_id
}
