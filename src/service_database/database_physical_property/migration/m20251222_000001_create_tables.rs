use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 pp_binary_nrtl_rk_entity 表
        manager
                   .create_table(
                       Table::create()
                           .table(PpBinaryNrtlRkEntity::Table)
                           .if_not_exists()

                           .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -nrtl_rk")
                           // 主键 id
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Id)
                                   .integer()
                                   .not_null()
                                   .auto_increment()
                                   .primary_key()
                                   .comment("主键"),
                           )
                           // compound_i_id（关联组分iid）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::CompoundIId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分iid"),
                           )
                           // compound_i（关联组分i的CASNO）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::CompoundI)
                                   .string() // 对应 VARCHAR，不指定长度则为默认长度（可改为 string_len(n) 定长）
                                   .not_null()
                                   .comment("关联组分i的CASNO"),
                           )
                           // component_j_id（关联组分jid）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::ComponentJId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分jid"),
                           )
                           // compound_j（关联组分j的casno）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::CompoundJ)
                                   .string()
                                   .not_null()
                                   .comment("关联组分j的casno"),
                           )
                           // AIJ（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Aij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // AJI（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Aji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // BIJ（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Bij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // BJI（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Bji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // CIJ（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Cij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // DIJ（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Dij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // EIJ（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Eij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // EJI（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Eji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // FIJ（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Fij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // FJI（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::Fji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // min_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::MinT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // max_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryNrtlRkEntity::MaxT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           .to_owned(),
                   )
                   .await?;

        manager
                    .create_table(
                        Table::create()
                            .table(PpBinaryNrtlEntity::Table)
                            .if_not_exists()
                            .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -nrtl")
                            // 主键及其他小写列（无变化）
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Id)
                                    .integer()
                                    .not_null()
                                    .auto_increment()
                                    .primary_key()
                                    .comment("主键"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::CompoundIId)
                                    .integer()
                                    .not_null()
                                    .comment("关联组分iid"),
                            )
                            // ... 其他无变化的列（compound_i、component_j_id、compound_j 等）
                            // 原大写列改为小写列（核心修改）
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Aij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Aji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Bij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Bji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Cij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Dij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Eij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Eji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Fij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .col(
                                ColumnDef::new(PpBinaryNrtlEntity::Fji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // ... min_t、max_t 列（无变化）
                            .to_owned(),
                    )
                    .await?;

        manager
                   .create_table(
                       Table::create()
                           .table(PpBinaryPrEntity::Table)
                           .if_not_exists()
                           .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -PR")
                           // 主键 id
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::Id)
                                   .integer()
                                   .not_null()
                                   .auto_increment()
                                   .primary_key()
                                   .comment("主键"),
                           )
                           // compound_i_id（关联组分iid）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::CompoundIId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分iid"),
                           )
                           // compound_i（关联组分i的CASNO）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::CompoundI)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分i的CASNO"),
                           )
                           // component_j_id（关联组分jid）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::ComponentJId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分jid"),
                           )
                           // compound_j（关联组分j的casno）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::CompoundJ)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分j的casno"),
                           )
                           // kaij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::Kaij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // kbij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::Kbij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // kcij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::Kcij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // min_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::MinT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // max_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPrEntity::MaxT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           .to_owned(),
                   )
                   .await?;

        manager
                   .create_table(
                       Table::create()
                           .table(PpBinaryPsrkEntity::Table)
                           .if_not_exists()
                           .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -psrk")
                           // 主键 id
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::Id)
                                   .integer()
                                   .not_null()
                                   .auto_increment()
                                   .primary_key()
                                   .comment("主键"),
                           )
                           // compound_i_id（关联组分iid）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::CompoundIId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分iid"),
                           )
                           // compound_i（关联组分i的CASNO）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::CompoundI)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分i的CASNO"),
                           )
                           // component_j_id（关联组分jid）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::ComponentJId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分jid"),
                           )
                           // compound_j（关联组分j的casno）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::CompoundJ)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分j的casno"),
                           )
                           // tij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::Tij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // tji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::Tji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // vij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::Vij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // vji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryPsrkEntity::Vji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           .to_owned(),
                   )
                   .await?;

        // 创建 pp_binary_rk_entity 表
        manager
                   .create_table(
                       Table::create()
                           .table(PpBinaryRkEntity::Table)
                           .if_not_exists()
                           .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -RK")
                           // 主键 id
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::Id)
                                   .integer()
                                   .not_null()
                                   .auto_increment()
                                   .primary_key()
                                   .comment("主键"),
                           )
                           // compound_i_id（关联组分iid）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::CompoundIId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分iid"),
                           )
                           // compound_i（关联组分i的CASNO）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::CompoundI)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分i的CASNO"),
                           )
                           // component_j_id（关联组分jid）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::ComponentJId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分jid"),
                           )
                           // compound_j（关联组分j的casno）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::CompoundJ)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分j的casno"),
                           )
                           // kaij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::Kaij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // kbij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::Kbij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // kcij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::Kcij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // min_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::MinT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // max_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryRkEntity::MaxT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           .to_owned(),
                   )
                   .await?;

        manager
                    .create_table(
                        Table::create()
                            .table(PpBinarySrkEntity::Table)
                            .if_not_exists()
                            .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -SRK")
                            // 主键 id
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::Id)
                                    .integer()
                                    .not_null()
                                    .auto_increment()
                                    .primary_key()
                                    .comment("主键"),
                            )
                            // compound_i_id（关联组分iid）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::CompoundIId)
                                    .integer()
                                    .not_null()
                                    .comment("关联组分iid"),
                            )
                            // compound_i（关联组分i的CASNO）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::CompoundI)
                                    .string() // 可改为 string_len(64) 指定固定长度
                                    .not_null()
                                    .comment("关联组分i的CASNO"),
                            )
                            // component_j_id（关联组分jid）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::ComponentJId)
                                    .integer()
                                    .not_null()
                                    .comment("关联组分jid"),
                            )
                            // compound_j（关联组分j的casno）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::CompoundJ)
                                    .string() // 可改为 string_len(64) 指定固定长度
                                    .not_null()
                                    .comment("关联组分j的casno"),
                            )
                            // kaij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::Kaij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // kbij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::Kbij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // kcij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::Kcij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // min_t（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::MinT)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // max_t（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinarySrkEntity::MaxT)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .to_owned(),
                    )
                    .await?;

        // 创建 pp_binary_uniquac_entity 表
        manager
                    .create_table(
                        Table::create()
                            .table(PpBinaryUniquacEntity::Table)
                            .if_not_exists()
                            .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -uniquac")
                            // 主键 id
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Id)
                                    .integer()
                                    .not_null()
                                    .auto_increment()
                                    .primary_key()
                                    .comment("主键"),
                            )
                            // compound_i_id（关联组分iid）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::CompoundIId)
                                    .integer()
                                    .not_null()
                                    .comment("关联组分iid"),
                            )
                            // compound_i（关联组分i的CASNO）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::CompoundI)
                                    .string() // 可改为 string_len(64) 指定固定长度
                                    .not_null()
                                    .comment("关联组分i的CASNO"),
                            )
                            // component_j_id（关联组分jid）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::ComponentJId)
                                    .integer()
                                    .not_null()
                                    .comment("关联组分jid"),
                            )
                            // compound_j（关联组分j的casno）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::CompoundJ)
                                    .string() // 可改为 string_len(64) 指定固定长度
                                    .not_null()
                                    .comment("关联组分j的casno"),
                            )
                            // aij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Aij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // aji（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Aji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // bij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Bij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // bji（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Bji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // cij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Cij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // cji（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Cji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // dij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Dij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // dji（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Dji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // eij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Eij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // eji（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Eji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // fij（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Fij)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // fji（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::Fji)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // min_t（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::MinT)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            // max_t（参数，默认值"0"）
                            .col(
                                ColumnDef::new(PpBinaryUniquacEntity::MaxT)
                                    .string()
                                    .default("0")
                                    .not_null()
                                    .comment("参数"),
                            )
                            .to_owned(),
                    )
                    .await?;

        // 创建 pp_binary_wilsion_entity 表
        manager
                   .create_table(
                       Table::create()
                           .table(PpBinaryWilsionEntity::Table)
                           .if_not_exists()
                           .comment("二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -wilsion")
                           // 主键 id
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Id)
                                   .integer()
                                   .not_null()
                                   .auto_increment()
                                   .primary_key()
                                   .comment("主键"),
                           )
                           // compound_i_id（关联组分iid）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::CompoundIId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分iid"),
                           )
                           // compound_i（关联组分i的CASNO）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::CompoundI)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分i的CASNO"),
                           )
                           // component_j_id（关联组分jid）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::ComponentJId)
                                   .integer()
                                   .not_null()
                                   .comment("关联组分jid"),
                           )
                           // compound_j（关联组分j的casno）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::CompoundJ)
                                   .string() // 可改为 string_len(64) 指定固定长度
                                   .not_null()
                                   .comment("关联组分j的casno"),
                           )
                           // aij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Aij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // aji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Aji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // bij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Bij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // bji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Bji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // cij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Cij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // cji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Cji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // dij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Dij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // dji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Dji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // eij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Eij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // eji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Eji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // fij（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Fij)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // fji（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::Fji)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // min_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::MinT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           // max_t（参数，默认值"0"）
                           .col(
                               ColumnDef::new(PpBinaryWilsionEntity::MaxT)
                                   .string()
                                   .default("0")
                                   .not_null()
                                   .comment("参数"),
                           )
                           .to_owned(),
                   )
                   .await?;

        manager
            .create_table(
                Table::create()
                    .table(PpCalcBasePropertyEntity::Table)
                    .if_not_exists()
                    .comment("物性方法- 物性系数表")
                    // 主键 id
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("主键"),
                    )
                    // name（名字）
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::Name)
                            .string()
                            .not_null() // TypeORM未指定nullable，默认非空
                            .comment("名字"),
                    )
                    // code（方法代号）
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::Code)
                            .string()
                            .not_null()
                            .comment("方法代号"),
                    )
                    // type（类型：数组, 双精度）
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::TypeStr)
                            .string()
                            .not_null()
                            .comment("类型：  数组, 双精度"),
                    )
                    // key（属性标识）
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::Key)
                            .string()
                            .not_null()
                            .comment("属性标识"),
                    )
                    // phase（气象，液相）
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::Phase)
                            .string()
                            .not_null()
                            .comment("气象，液相"),
                    )
                    // mixture（混合物=1 纯组分=0）
                    .col(
                        ColumnDef::new(PpCalcBasePropertyEntity::Mixture)
                            .integer()
                            .not_null()
                            .comment("混合物=1 纯组分=0"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除顺序通常与创建顺序相反

        manager
            .drop_table(Table::drop().table(PpBinaryNrtlRkEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinaryNrtlEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinaryPrEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinaryPsrkEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinaryRkEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinarySrkEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinaryUniquacEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PpBinaryWilsionEntity::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(PpCalcBasePropertyEntity::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

/// 表/列名枚举（与实体字段对应，自动转为蛇形命名）
#[derive(Iden)]
enum PpBinaryNrtlRkEntity {
    Table,
    // 列名：id
    Id,
    CompoundIId,
    CompoundI,
    ComponentJId,
    CompoundJ,
    Aij,
    Aji,
    Bij,
    Bji,
    Cij,
    Dij,
    Eij,
    Eji,
    Fij,
    Fji,
    MinT,
    MaxT,
}

#[derive(Iden)]
enum PpBinaryNrtlEntity {
    Table,
    Id,
    CompoundIId,
    CompoundI,
    ComponentJId,
    CompoundJ,
    Aij,
    Aji,
    Bij,
    Bji,
    Cij,
    Dij,
    Eij,
    Eji,
    Fij,
    Fji,
    MinT,
    MaxT,
}
#[derive(Iden)]
enum PpBinaryPrEntity {
    // 表名：pp_binary_pr_entity
    Table,
    // 列名：id
    Id,
    // 列名：compound_i_id
    CompoundIId,
    // 列名：compound_i
    CompoundI,
    // 列名：component_j_id
    ComponentJId,
    // 列名：compound_j
    CompoundJ,
    // 列名：kaij（自动转为小写）
    Kaij,
    // 列名：kbij（自动转为小写）
    Kbij,
    // 列名：kcij（自动转为小写）
    Kcij,
    // 列名：min_t
    MinT,
    // 列名：max_t
    MaxT,
}

#[derive(Iden)]
enum PpBinaryPsrkEntity {
    // 表名：pp_binary_psrk_entity
    Table,
    // 列名：id
    Id,
    // 列名：compound_i_id
    CompoundIId,
    // 列名：compound_i
    CompoundI,
    // 列名：component_j_id
    ComponentJId,
    // 列名：compound_j
    CompoundJ,
    // 列名：tij（自动转为小写）
    Tij,
    // 列名：tji（自动转为小写）
    Tji,
    // 列名：vij（自动转为小写）
    Vij,
    // 列名：vji（自动转为小写）
    Vji,
}

#[derive(Iden)]
enum PpBinaryRkEntity {
    // 表名：pp_binary_rk_entity
    Table,
    // 列名：id
    Id,
    // 列名：compound_i_id
    CompoundIId,
    // 列名：compound_i
    CompoundI,
    // 列名：component_j_id
    ComponentJId,
    // 列名：compound_j
    CompoundJ,
    // 列名：kaij（自动转为小写）
    Kaij,
    // 列名：kbij（自动转为小写）
    Kbij,
    // 列名：kcij（自动转为小写）
    Kcij,
    // 列名：min_t
    MinT,
    // 列名：max_t
    MaxT,
}

/// 表/列名枚举（自动转为小写列名，无需自定义大写注解）
#[derive(Iden)]
enum PpBinarySrkEntity {
    // 表名：pp_binary_srk_entity
    Table,
    // 列名：id
    Id,
    // 列名：compound_i_id
    CompoundIId,
    // 列名：compound_i
    CompoundI,
    // 列名：component_j_id
    ComponentJId,
    // 列名：compound_j
    CompoundJ,
    // 列名：kaij（自动转为小写）
    Kaij,
    // 列名：kbij（自动转为小写）
    Kbij,
    // 列名：kcij（自动转为小写）
    Kcij,
    // 列名：min_t
    MinT,
    // 列名：max_t
    MaxT,
}

/// 表/列名枚举（自动转为小写列名，无需自定义大写注解）
#[derive(Iden)]
enum PpBinaryUniquacEntity {
    // 表名：pp_binary_uniquac_entity
    Table,
    // 列名：id
    Id,
    // 列名：compound_i_id
    CompoundIId,
    // 列名：compound_i
    CompoundI,
    // 列名：component_j_id
    ComponentJId,
    // 列名：compound_j
    CompoundJ,
    // 列名：aij（自动转为小写）
    Aij,
    // 列名：aji（自动转为小写）
    Aji,
    // 列名：bij（自动转为小写）
    Bij,
    // 列名：bji（自动转为小写）
    Bji,
    // 列名：cij（自动转为小写）
    Cij,
    // 列名：cji（自动转为小写）
    Cji,
    // 列名：dij（自动转为小写）
    Dij,
    // 列名：dji（自动转为小写）
    Dji,
    // 列名：eij（自动转为小写）
    Eij,
    // 列名：eji（自动转为小写）
    Eji,
    // 列名：fij（自动转为小写）
    Fij,
    // 列名：fji（自动转为小写）
    Fji,
    // 列名：min_t
    MinT,
    // 列名：max_t
    MaxT,
}

#[derive(Iden)]
enum PpBinaryWilsionEntity {
    // 表名：pp_binary_wilsion_entity
    Table,
    // 列名：id
    Id,
    // 列名：compound_i_id
    CompoundIId,
    // 列名：compound_i
    CompoundI,
    // 列名：component_j_id
    ComponentJId,
    // 列名：compound_j
    CompoundJ,
    // 列名：aij（自动转为小写）
    Aij,
    // 列名：aji（自动转为小写）
    Aji,
    // 列名：bij（自动转为小写）
    Bij,
    // 列名：bji（自动转为小写）
    Bji,
    // 列名：cij（自动转为小写）
    Cij,
    // 列名：cji（自动转为小写）
    Cji,
    // 列名：dij（自动转为小写）
    Dij,
    // 列名：dji（自动转为小写）
    Dji,
    // 列名：eij（自动转为小写）
    Eij,
    // 列名：eji（自动转为小写）
    Eji,
    // 列名：fij（自动转为小写）
    Fij,
    // 列名：fji（自动转为小写）
    Fji,
    // 列名：min_t
    MinT,
    // 列名：max_t
    MaxT,
}
#[derive(Iden)]
enum PpCalcBasePropertyEntity {
    // 表名：pp_calc_base_physical
    Table,
    // 列名：id
    Id,
    // 列名：name
    Name,
    // 列名：code
    Code,
    // 列名：type
    TypeStr,
    // 列名：key
    Key,
    // 列名：phase
    Phase,
    // 列名：mixture
    Mixture,
}
