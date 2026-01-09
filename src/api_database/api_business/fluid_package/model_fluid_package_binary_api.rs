use napi_derive::napi;
use crate::service_database::database_business::service::fluid_package::model_fluid_package_binary_service::*;

use crate::error_handle::err_handle::*;
use futures::try_join;
use napi::Result;
#[napi(namespace = "modelFluidPackageBinary")]
pub async fn update_binary_interaction_parameters(
    func_code: String,
    data_json: String,
) -> Result<bool> {
    // 1. 解析前端传来的 JSON 字符串
    let datas: Vec<BinaryParameterDto> = serde_json::from_str(&data_json)
        .map_err(|e| napi::Error::from_reason(format!("JSON 解析失败: {}", e)))?;

    // 2. 根据 func_code 分发到不同的 Service
    // 这里的字符串匹配逻辑与 TS 端保持一致
    match func_code.to_uppercase().as_str() {
        "PR" => PrService::batch_save(datas).await.map_err(handle_db_err)?,
        "RK" => RkService::batch_save(datas).await.map_err(handle_db_err)?,
        "SRK" => SrkService::batch_save(datas).await.map_err(handle_db_err)?,
        "NRTL" => NrtlService::batch_save(datas)
            .await
            .map_err(handle_db_err)?,
        "NRTL-RK" => NrtlRkService::batch_save(datas)
            .await
            .map_err(handle_db_err)?,
        "WILSON" => WilsonService::batch_save(datas)
            .await
            .map_err(handle_db_err)?,
        "UNIQUAC" => UniquacService::batch_save(datas)
            .await
            .map_err(handle_db_err)?,
        "PSRK" => PsrkService::batch_save(datas)
            .await
            .map_err(handle_db_err)?,
        _ => {
            return Err(napi::Error::from_reason(format!(
                "不支持的物性方法: {}",
                func_code
            )));
        }
    };

    Ok(true)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn get_binary_parameters_by_package(
    func_code: String,
    package_id: String,
) -> napi::Result<String> {
    // 根据 func_code 获取数据并序列化为 JSON 字符串返回给 Node.js
    let result = match func_code.to_uppercase().as_str() {
        "PR" => serde_json::to_string(
            &PrService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "RK" => serde_json::to_string(
            &RkService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "SRK" => serde_json::to_string(
            &SrkService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "NRTL" => serde_json::to_string(
            &NrtlService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "NRTL-RK" => serde_json::to_string(
            &NrtlRkService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "WILSON" => serde_json::to_string(
            &WilsonService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "UNIQUAC" => serde_json::to_string(
            &UniquacService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        "PSRK" => serde_json::to_string(
            &PsrkService::find_by_package_id(package_id)
                .await
                .map_err(handle_db_err)?,
        ),
        _ => return Err(napi::Error::from_reason("Unknown funcCode")),
    }
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(result)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn get_binary_parameters_by_ids(
    func_code: String,
    ids: Vec<String>,
) -> napi::Result<String> {
    // 根据 func_code 获取数据并序列化为 JSON 字符串返回给 Node.js
    let result = match func_code.to_uppercase().as_str() {
        "PR" => serde_json::to_string(&PrService::find_by_ids(ids).await.map_err(handle_db_err)?),
        "RK" => serde_json::to_string(&RkService::find_by_ids(ids).await.map_err(handle_db_err)?),
        "SRK" => serde_json::to_string(&SrkService::find_by_ids(ids).await.map_err(handle_db_err)?),
        "NRTL" => {
            serde_json::to_string(&NrtlService::find_by_ids(ids).await.map_err(handle_db_err)?)
        }
        "NRTL-RK" => serde_json::to_string(
            &NrtlRkService::find_by_ids(ids)
                .await
                .map_err(handle_db_err)?,
        ),
        "WILSON" => serde_json::to_string(
            &WilsonService::find_by_ids(ids)
                .await
                .map_err(handle_db_err)?,
        ),
        "UNIQUAC" => serde_json::to_string(
            &UniquacService::find_by_ids(ids)
                .await
                .map_err(handle_db_err)?,
        ),
        "PSRK" => {
            serde_json::to_string(&PsrkService::find_by_ids(ids).await.map_err(handle_db_err)?)
        }
        _ => return Err(napi::Error::from_reason("Unknown funcCode")),
    }
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(result)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn delete_binary_parameters(func_code: String, package_id: String) -> napi::Result<bool> {
    let rows_affected = match func_code.to_uppercase().as_str() {
        "PR" => PrService::delete_by_package_id(package_id).await,
        "RK" => RkService::delete_by_package_id(package_id).await,
        "SRK" => SrkService::delete_by_package_id(package_id).await,
        "NRTL" => NrtlService::delete_by_package_id(package_id).await,
        "NRTL-RK" => NrtlRkService::delete_by_package_id(package_id).await,
        "WILSON" => WilsonService::delete_by_package_id(package_id).await,
        "UNIQUAC" => UniquacService::delete_by_package_id(package_id).await,
        "PSRK" => PsrkService::delete_by_package_id(package_id).await,

        _ => return Err(napi::Error::from_reason("Unknown funcCode")),
    }
    .map_err(handle_db_err)?;

    Ok(rows_affected)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn delete_binary_parameters_by_package_id(package_id: String) -> napi::Result<()> {
    try_join!(
        PrService::delete_by_package_id(package_id.clone()),
        RkService::delete_by_package_id(package_id.clone()),
        SrkService::delete_by_package_id(package_id.clone()),
        NrtlService::delete_by_package_id(package_id.clone()),
        NrtlRkService::delete_by_package_id(package_id.clone()),
        WilsonService::delete_by_package_id(package_id.clone()),
        UniquacService::delete_by_package_id(package_id.clone()),
        PsrkService::delete_by_package_id(package_id),
    )
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(())
}
