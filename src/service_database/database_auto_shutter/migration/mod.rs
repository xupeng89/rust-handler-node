pub use sea_orm_migration::prelude::*;

mod m20251203_000001_create_tables;
mod m20251229_000001_alert_tables;
mod m20260105_000001_alert_tables;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251203_000001_create_tables::Migration),
            Box::new(m20251229_000001_alert_tables::Migration),
            Box::new(m20260105_000001_alert_tables::Migration),
        ]
    }
}
