use crate::setup_db_instance;

setup_db_instance!(
    business,
    BUSINESS_DB,
    "BusinessDB",
    crate::service_database::database_business::migration::Migrator
);
