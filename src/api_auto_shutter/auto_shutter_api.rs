use napi::Result;
use napi_derive::napi;

use crate::api_cache::err_handle::*;
use crate::database_auto_shutter::service::auto_shutter_service as local_service;
use crate::database_cache::service::auto_shutter_cache_service as cache_service;

#[napi(namespace = "autoShutter")]
pub async fn sync_cache_to_local_api() -> Result<()> {
    // ⚠️ 改用 eprintln! 确保立即输出
    eprintln!("🚀 [同步] 开始从缓存数据库读取自动快门数据...");

    let sync_data = cache_service::get_all_model_auto_shutter_entity_cache()
        .await
        .map_err(SyncError::CacheReadError)?;

    if sync_data.is_empty() {
        eprintln!("⚠️ [同步] 缓存数据库无数据可同步到本地");
        return Ok(());
    }

    eprintln!(
        "📥 [同步] 从缓存读取到 {} 条数据，准备同步到本地数据库...",
        sync_data.len()
    );

    // 假设 local_service 接受引用或 clone，保持你原逻辑
    local_service::read_current_model_auto_shutter_entity(sync_data.clone())
        .await
        .map_err(SyncError::LocalWriteError)?;

    eprintln!("✅ [同步] 成功同步 {} 条数据到本地数据库", sync_data.len());
    Ok(())
}

#[napi(namespace = "autoShutter")]
pub async fn sync_local_to_cache_api() -> Result<()> {
    // ⚠️ 改用 eprintln!
    eprintln!("🚀 [回写] 开始从本地数据库读取自动快门数据...");

    let sync_data = local_service::get_current_all_model_auto_shutter_entity()
        .await
        .map_err(SyncError::LocalReadError)?;

    if sync_data.is_empty() {
        eprintln!("⚠️ [回写] 本地数据库无数据可回写缓存");
        return Ok(());
    }

    eprintln!(
        "📤 [回写] 从本地读取到 {} 条数据，准备回写缓存数据库...",
        sync_data.len()
    );

    cache_service::read_model_auto_shutter_entity_cache(sync_data.clone())
        .await
        .map_err(SyncError::CacheWriteError)?;

    eprintln!("✅ [回写] 成功回写 {} 条数据到缓存数据库", sync_data.len());
    Ok(())
}
