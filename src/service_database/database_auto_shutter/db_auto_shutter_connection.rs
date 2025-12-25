use crate::setup_db_instance;

setup_db_instance!(
    auto_shutter,
    AUTO_SHUTTER_DB,
    "AutoShutterDB",
    crate::service_database::database_auto_shutter::migration::Migrator
);
