use napi::Result;
use napi_derive::napi;
use serde_json::Value;

// 引入 Henry 相关的 Service 函数和 DTO
use crate::service_database::database_physical_property::service::physical_henry_service::*;
// 引入项目统一的错误处理器
use crate::error_handle::err_handle::handle_db_err;

#[napi(namespace = "heryDetail")] // 注意：这里保持和你 DTO 定义一致的命名空间拼写
/// 同步亨利详细数据 (包含差集删除、更新和插入)
pub async fn sync_pp_henry_detail_data_api(data: Vec<Value>) -> Result<()> {
    sync_pp_henry_detail_data(data)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

#[napi(namespace = "heryDetail")]
/// 根据溶质(I)列表和溶剂(J)列表查询亨利常数记录
pub async fn query_pp_component_henry_detail_data_by_i_or_j_api(
    ids_i: Vec<String>,
    ids_j: Vec<String>,
) -> Result<Vec<HenryDetailDTO>> {
    query_pp_component_henry_detail_data_by_i_or_j(ids_i, ids_j)
        .await
        .map_err(handle_db_err)
}
