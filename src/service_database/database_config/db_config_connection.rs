use crate::setup_db_instance;

setup_db_instance!(
    config,
    CONFIG_DB,
    "ConfigDB",
    crate::service_database::database_config::migration::Migrator
);
