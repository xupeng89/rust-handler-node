use sea_orm_migration::prelude::*;

pub async fn drop_tables(
    manager: &SchemaManager<'_>,
    tables: &[&(dyn Iden + Sync)],
) -> Result<(), DbErr> {
    for table in tables {
        manager
            .drop_table(
                Table::drop()
                    // 必须先转为字符串再转为 Alias，因为 &dyn Iden 不直接支持 IntoTableRef
                    .table(Alias::new(table.to_string()))
                    .to_owned(),
            )
            .await?;
    }
    Ok(())
}
