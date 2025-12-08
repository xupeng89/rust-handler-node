use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// 对应数据库的表名和列名
// 注意：ConfConfigEntity 应该与上面的实体名称一致
#[derive(Iden)]
enum ConfConfigEntity {
    Table,
    Id,
    PropertyParams,
    ControlParams,
    RateParams,
    FlashParams,
    FilterLabelParams,
    ModelState,
    ShowLabelParams,
    RangeStatus,
    AutoShutterParams,
    OilParams,
}

#[derive(Iden)]
enum ConfFunctionPicEntity {
    Table,   // 对应表名 conf_function_pic
    Id,      // 对应 @PrimaryGeneratedColumn
    Name,    // 对应 name: string
    Picture, // 对应 picture: string
    Code,    // 对应 code: string
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // --- 对应 TypeORM: conf_config ---
        manager
            .create_table(
                Table::create()
                    .table(ConfConfigEntity::Table)
                    .if_not_exists()
                    // id: number @PrimaryGeneratedColumn
                    .col(
                        ColumnDef::new(ConfConfigEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // propertyParams: string @Column(name: "property_params", type: "text", default: "")
                    .col(
                        ColumnDef::new(ConfConfigEntity::PropertyParams)
                            .text()
                            .default(""),
                    )
                    // controlParams: string @Column(name: "control_params", type: "text", default: "")
                    .col(
                        ColumnDef::new(ConfConfigEntity::ControlParams)
                            .text()
                            .default(""),
                    )
                    // rateParams: string @Column(name: "rate_params", type: "text", default: "")
                    .col(
                        ColumnDef::new(ConfConfigEntity::RateParams)
                            .text()
                            .default(""),
                    )
                    // flashParams: string @Column(name: "flash_params", type: "text", default: "")
                    .col(
                        ColumnDef::new(ConfConfigEntity::FlashParams)
                            .text()
                            .default(""),
                    )
                    // filterLabelParams: string @Column(name: "filter_label_params", type: "text", default: "")
                    .col(
                        ColumnDef::new(ConfConfigEntity::FilterLabelParams)
                            .text()
                            .default(""),
                    )
                    // modelState: number @Column(name: "model_state", type: "int", default: 0)
                    .col(
                        ColumnDef::new(ConfConfigEntity::ModelState)
                            .integer()
                            .default(0),
                    )
                    // showLabelParams: string @Column(name: "show_label_params", type: "text", default: "")
                    .col(
                        ColumnDef::new(ConfConfigEntity::ShowLabelParams)
                            .text()
                            .default(r#"{"globalLabel":[],"flowSheetLabel":[]}"#),
                    )
                    // rangeStatus: number @Column(name: "range_status", type: "int", default: 1)
                    .col(
                        ColumnDef::new(ConfConfigEntity::RangeStatus)
                            .integer()
                            .default(1),
                    )
                    // autoShutterParams: string @Column(name: "auto_shutter_params", type: "text", default: JSON.stringify({...}))
                    .col(
                        ColumnDef::new(ConfConfigEntity::AutoShutterParams)
                            .text()
                            // 注意：在 Rust 中 default 必须是字面量字符串
                            .default(r#"{"autoShutter":1,"autoTimeInterval":30,"autoCount":60}"#),
                    )
                    // oilParams: string @Column(name: "oil_params", type: "text", default: JSON.stringify([]))
                    .col(
                        ColumnDef::new(ConfConfigEntity::OilParams)
                            .text()
                            .default("[]"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(
                        // TypeORM: @Entity({ name: "conf_function_pic" })
                        ConfFunctionPicEntity::Table,
                    )
                    .if_not_exists()
                    // id: number @PrimaryGeneratedColumn
                    .col(
                        ColumnDef::new(ConfFunctionPicEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("主键"), // TypeORM: comment: "主键"
                    )
                    // name: string @Column(type: "varchar", comment: "公式名称")
                    .col(
                        ColumnDef::new(ConfFunctionPicEntity::Name)
                            .string_len(255) // 假设 varchar 长度为 255
                            .not_null() // 假设 name 不允许为空
                            .comment("公式名称"),
                    )
                    // picture: string @Column(type: "text", comment: "图片数据，base64")
                    .col(
                        ColumnDef::new(ConfFunctionPicEntity::Picture)
                            .text()
                            .not_null() // 假设 picture 不允许为空
                            .comment("图片数据，base64"),
                    )
                    // code: string @Column(type: "varchar", comment: "唯一ID")
                    .col(
                        ColumnDef::new(ConfFunctionPicEntity::Code)
                            .string_len(50) // 假设 code 长度为 50，作为唯一 ID
                            .not_null() // 假设 code 不允许为空
                            .comment("唯一ID"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConfConfigEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ConfFunctionPicEntity::Table).to_owned())
            .await?;
        Ok(())
    }
}
