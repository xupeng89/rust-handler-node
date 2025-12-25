use crate::setup_db_instance;

setup_db_instance!(
    shutter,
    SHUTTER_DB,
    "ShutterDB",
    crate::service_database::database_shutter::migration::Migrator
);
