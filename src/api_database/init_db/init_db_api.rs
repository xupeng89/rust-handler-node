use napi::Result;
use napi_derive::napi;

use crate::service_database::database_cache::db_cache_connection::{DbStats, get_cache_db_stats};

#[napi(namespace = "initDB")]
/// 初始化缓存数据库
pub async fn init_cache_db() -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_cache::db_cache_connection::get_cache_db().await {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

#[napi(namespace = "initDB")]
/// 初始化快照数据库
pub async fn init_shutter_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_shutter::db_shutter_connection::initialize_shutter_db(
        url,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_auto_shutter_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_auto_shutter::db_auto_shutter_connection::initialize_auto_shutter_db(url)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_config_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_config::db_config_connection::initialize_config_db(url)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_physical_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_physical_property::db_physical_property_connection::initialize_physical_property_db(url)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_business_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_business::db_business_connection::initialize_business_db(url)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

/// 关闭数据库使用
#[napi(namespace = "initDB")]
pub async fn close_all_dbs() -> Result<()> {
    // 关闭业务数据库 business
    crate::service_database::database_business::db_business_connection::close_business_db().await;
    // 关闭auto_shutter数据库
    crate::service_database::database_auto_shutter::db_auto_shutter_connection::close_auto_shutter_db().await;

    // 关闭shutter数据库
    crate::service_database::database_shutter::db_shutter_connection::close_shutter_db().await;

    // 关闭config数据库
    crate::service_database::database_config::db_config_connection::close_config_db().await;
    // 关闭物性数据库
    crate::service_database::database_physical_property::db_physical_property_connection::close_physical_property_db().await;

    Ok(())
}

/// 移动数据方法，目前只移动business数据库
#[napi(namespace = "initDB")]
pub async fn move_db_by_url(target_path: String) -> Result<()> {
    match crate::service_database::database_business::db_business_connection::backup_business_db(
        target_path,
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(napi::Error::from_reason(format!("Database Error: {}", e))),
    }
}

#[napi(namespace = "initDB")]
pub async fn get_cache_db_stats_api() -> Result<DbStats> {
    get_cache_db_stats().await
}
