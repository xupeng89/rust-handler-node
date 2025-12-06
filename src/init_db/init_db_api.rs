use napi::Result;
use napi_derive::napi;
#[napi(namespace = "initDB")]
/// 初始化缓存数据库
pub async fn init_cache_db() -> Result<()> {
    // 初始化cache数据库
    match crate::database_cache::db_cache_connection::get_cache_db().await {
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
    match crate::database_shutter::db_shutter_connection::initialize_shutter_db(url).await {
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
    match crate::database_auto_shutter::db_auto_shutter_connection::initialize_auto_shutter_db(url)
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("init_auto_shutter_db error: {}", e);
        }
    };
    Ok(())
}

// #[napi(namespace = "initDB")]
// /// 初始化自动快照数据库
// pub async fn init_config_db(url: String) -> Result<()> {
//     // 初始化cache数据库
//     match crate::database_config::db_config_connection::initialize_config_db(url).await {
//         Ok(_) => {}
//         Err(e) => {
//             eprintln!("init_config_db error: {}", e);
//         }
//     };
//     Ok(())
// }
