use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum ConfModelEntity {
    Table,                   // 对应表名 conf_model_entity
    Id,                      // 主键
    StandardTemperature,     // standard_temperature
    StandardTemperatureUnit, // standard_temperature_unit
    StandardPressure,        // standard_pressure
    StandardPressureUnit,    // standard_pressure_unit
    GridState,               // grid_state
    GridColor,               // grid_color
    GridSize,                // grid_size
    CreateAt,                // create_at
    UpdateAt,                // update_at
    Language,                // language
}

#[derive(Iden)]
enum ConfGraphicCustomEntity {
    Table,      // 对应表名 conf_graphic_custom_entity
    Id,         // 主键
    Name,       // 公式名称
    Code,       // 编码
    CustomType, // custom_type
    Arithmetic, // 算术/公式
    Size,       // 尺寸
    Svg,        // svg内容
    Ports,      // 端口
    WindowSize, // window_size
    DndType,    // dnd_type
}

// 对应数据库的表名和列名
// 注意：ConfConfigEntity 应该与上面的实体名称一致
#[derive(Iden)]
enum ConfConfigEntity {
    Table,
    Id,
    Name,
    Code,
    Value,
    ValueType,
}

#[derive(Iden)]
enum ConfFunctionPicEntity {
    Table,   // 对应表名 conf_function_pic_entity
    Id,      // 对应 @PrimaryGeneratedColumn
    Name,    // 对应 name: string
    Picture, // 对应 picture: string
    Code,    // 对应 code: string
}

#[derive(Iden)]
enum ConfPfModelParamsEntity {
    Table,      // 表名：conf_pf_model_params_entity
    Id,         // 主键（i32 自增）
    Code,       // code
    Name,       // name
    SolverType, // solver_type
    Params,     // params（Text 类型）
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
                    .col(ColumnDef::new(ConfConfigEntity::Value).text().default(""))
                    .comment("配置内容")
                    .col(
                        ColumnDef::new(ConfConfigEntity::Name)
                            .string_len(255) // 假设 varchar 长度为 255
                            .not_null() // 假设 name 不允许为空
                            .comment("配置名称"),
                    )
                    .col(
                        ColumnDef::new(ConfConfigEntity::Code)
                            .string_len(255) // 假设 varchar 长度为 255
                            .not_null() // 假设 name 不允许为空
                            .comment("配置编码"),
                    )
                    .col(
                        ColumnDef::new(ConfConfigEntity::ValueType)
                            .string_len(255) // 假设 varchar 长度为 255
                            .not_null() // 假设 name 不允许为空
                            .comment("配置值类型"),
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
        // 3. 创建 conf_graphic_custom 表（对应原 ConfFunctionPic 实体）
        manager
            .create_table(
                Table::create()
                    .table(ConfGraphicCustomEntity::Table)
                    .if_not_exists()
                    // 主键 id
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("主键"),
                    )
                    //  name
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Name)
                            .string_len(255)
                            .not_null()
                            .comment("名称"),
                    )
                    // 编码 code
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Code)
                            .string_len(255)
                            .not_null()
                            .comment("编码"),
                    )
                    // 自定义类型 custom_type
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::CustomType)
                            .string_len(255)
                            .not_null()
                            .comment("自定义类型"),
                    )
                    // 算术/公式 arithmetic
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Arithmetic)
                            .string_len(255)
                            .not_null()
                            .comment("算术/公式"),
                    )
                    // 尺寸 size
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Size)
                            .string_len(255)
                            .not_null()
                            .comment("尺寸"),
                    )
                    // svg内容（文本类型）
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Svg)
                            .text()
                            .not_null()
                            .comment("SVG内容"),
                    )
                    // 端口 ports
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::Ports)
                            .string_len(255)
                            .not_null()
                            .comment("端口配置"),
                    )
                    // 窗口尺寸 window_size
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::WindowSize)
                            .string_len(255)
                            .not_null()
                            .comment("窗口尺寸"),
                    )
                    // 拖拽类型 dnd_type
                    .col(
                        ColumnDef::new(ConfGraphicCustomEntity::DndType)
                            .string_len(255)
                            .not_null()
                            .comment("拖拽类型"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ConfModelEntity::Table)
                    .if_not_exists()
                    // 主键 id（字符串类型）
                    .col(
                        ColumnDef::new(ConfModelEntity::Id)
                            .string_len(64) // 字符串主键，建议长度64
                            .not_null()
                            .primary_key()
                            .comment("主键（字符串类型）"),
                    )
                    // 标准温度
                    .col(
                        ColumnDef::new(ConfModelEntity::StandardTemperature)
                            .double()
                            .not_null()
                            .comment("标准温度值"),
                    )
                    // 标准温度单位
                    .col(
                        ColumnDef::new(ConfModelEntity::StandardTemperatureUnit)
                            .string_len(50)
                            .not_null()
                            .comment("标准温度单位"),
                    )
                    // 标准压力
                    .col(
                        ColumnDef::new(ConfModelEntity::StandardPressure)
                            .double()
                            .not_null()
                            .comment("标准压力值"),
                    )
                    // 标准压力单位
                    .col(
                        ColumnDef::new(ConfModelEntity::StandardPressureUnit)
                            .string_len(50)
                            .not_null()
                            .comment("标准压力单位"),
                    )
                    // 网格状态
                    .col(
                        ColumnDef::new(ConfModelEntity::GridState)
                            .string_len(50)
                            .not_null()
                            .comment("网格状态"),
                    )
                    // 网格颜色
                    .col(
                        ColumnDef::new(ConfModelEntity::GridColor)
                            .string_len(50)
                            .not_null()
                            .comment("网格颜色"),
                    )
                    // 网格尺寸
                    .col(
                        ColumnDef::new(ConfModelEntity::GridSize)
                            .string_len(50)
                            .not_null()
                            .comment("网格尺寸"),
                    )
                    // 创建时间
                    .col(
                        ColumnDef::new(ConfModelEntity::CreateAt)
                            .date_time() // 对应 NaiveDateTime
                            .not_null()
                            .comment("创建时间"),
                    )
                    // 更新时间
                    .col(
                        ColumnDef::new(ConfModelEntity::UpdateAt)
                            .date_time() // 对应 NaiveDateTime
                            .not_null()
                            .comment("更新时间"),
                    )
                    // 语言
                    .col(
                        ColumnDef::new(ConfModelEntity::Language)
                            .string_len(50)
                            .not_null()
                            .comment("语言类型"),
                    )
                    .to_owned(),
            )
            .await?;
        // 5. 创建 conf_pf_model_params_entity 表（新增：数学求解方法配置表）
        manager
            .create_table(
                Table::create()
                    .table(ConfPfModelParamsEntity::Table)
                    .if_not_exists()
                    // 主键 id（i32 自增）
                    .col(
                        ColumnDef::new(ConfPfModelParamsEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("主键"),
                    )
                    // code 字段
                    .col(
                        ColumnDef::new(ConfPfModelParamsEntity::Code)
                            .string_len(255)
                            .not_null()
                            .comment("编码"),
                    )
                    // name 字段
                    .col(
                        ColumnDef::new(ConfPfModelParamsEntity::Name)
                            .string_len(255)
                            .not_null()
                            .comment("名称"),
                    )
                    // solver_type 字段
                    .col(
                        ColumnDef::new(ConfPfModelParamsEntity::SolverType)
                            .string_len(255)
                            .not_null()
                            .comment("求解器类型"),
                    )
                    // params 字段（Text 类型）
                    .col(
                        ColumnDef::new(ConfPfModelParamsEntity::Params)
                            .text()
                            .not_null()
                            .comment("求解参数（JSON/文本）"),
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

        manager
            .drop_table(
                Table::drop()
                    .table(ConfGraphicCustomEntity::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ConfModelEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(ConfPfModelParamsEntity::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
