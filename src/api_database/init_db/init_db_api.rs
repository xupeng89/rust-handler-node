use napi::Result;
use napi_derive::napi;
#[napi(namespace = "initDB")]
/// 初始化缓存数据库
pub async fn init_cache_db() -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_cache::db_cache_connection::get_cache_db().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("init_cache_db error: {}", e);
        }
    };
    Ok(())
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
        Ok(_) => {}
        Err(e) => {
            eprintln!("init_shutter_db error: {}", e);
        }
    };
    Ok(())
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_auto_shutter_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_auto_shutter::db_auto_shutter_connection::initialize_auto_shutter_db(url)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("init_auto_shutter_db error: {}", e);
        }
    };
    Ok(())
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_config_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_config::db_config_connection::initialize_config_db(url)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("init_config_db error: {}", e);
        }
    };
    Ok(())
}

#[napi(namespace = "initDB")]
/// 初始化自动快照数据库
pub async fn init_physical_db(url: String) -> Result<()> {
    // 初始化cache数据库
    match crate::service_database::database_physical_property::db_physical_property_connection::initialize_physical_property_db(url)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("init_physical_db error: {}", e);
        }
    };
    Ok(())
}

/// 关闭数据库使用
#[napi(namespace = "initDB")]
pub async fn close_all_dbs() -> Result<()> {
    // 关闭config数据库
    crate::service_database::database_config::db_config_connection::close_config_db().await;

    // 关闭物性数据库
    crate::service_database::database_physical_property::db_physical_property_connection::close_physical_property_db().await;

    // 关闭shutter数据库
    crate::service_database::database_shutter::db_shutter_connection::close_shutter_db().await;

    // 关闭auto_shutter数据库
    crate::service_database::database_auto_shutter::db_auto_shutter_connection::close_auto_shutter_db().await;

    Ok(())
}
