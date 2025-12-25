use crate::setup_db_instance;

setup_db_instance!(
    physical_property,
    PHYSICAL_PROPERTY_DB,
    "PhysicalPropertyDB",
    crate::service_database::database_physical_property::migration::Migrator
);
