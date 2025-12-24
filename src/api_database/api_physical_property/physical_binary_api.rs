use napi::Result;
use napi_derive::napi;
use serde_json::Value;

// 引入分发函数
use crate::service_database::database_physical_property::service::physical_binary_service::{
    dispatch_query_request, dispatch_sync_request,
};
// 引入错误处理
use crate::error_handle::err_handle::handle_db_err;

/// 批量同步接口
#[napi(namespace = "physicalBinarySync")]
pub async fn sync_binary_data_api(func_code: String, data: Vec<Value>) -> Result<()> {
    // 调用分发逻辑，并将 SeaORM DbErr 转换为 napi Error
    dispatch_sync_request(&func_code, data)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

/// 批量查询接口
#[napi(namespace = "physicalBinarySync")]
pub async fn query_binary_data_api(func_code: String, ids: Vec<String>) -> Result<Vec<Value>> {
    // 调用分发逻辑
    let result = dispatch_query_request(&func_code, ids)
        .await
        .map_err(handle_db_err)?;

    Ok(result)
}
