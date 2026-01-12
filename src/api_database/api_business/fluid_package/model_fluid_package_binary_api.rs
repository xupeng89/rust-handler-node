use napi_derive::napi;
use crate::service_database::database_business::service::fluid_package::model_fluid_package_binary_service::*;

use crate::error_handle::err_handle::*;
use futures::try_join;
use napi::Result;
use serde::{Deserialize, Serialize};
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
pub async fn get_binary_parameters_by_fluid_package_id_default(
    func_code: String,
    package_id: String,
    is_default: u32,
) -> napi::Result<String> {
    // 根据 func_code 获取数据并序列化为 JSON 字符串返回给 Node.js
    let result = match func_code.to_uppercase().as_str() {
        "PR" => serde_json::to_string(
            &PrService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "RK" => serde_json::to_string(
            &RkService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "SRK" => serde_json::to_string(
            &SrkService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "NRTL" => serde_json::to_string(
            &NrtlService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "NRTL-RK" => serde_json::to_string(
            &NrtlRkService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "WILSON" => serde_json::to_string(
            &WilsonService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "UNIQUAC" => serde_json::to_string(
            &UniquacService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
        "PSRK" => serde_json::to_string(
            &PsrkService::find_by_package_id_and_default(package_id, is_default)
                .await
                .map_err(handle_db_err)?,
        ),
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

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn delete_binary_parameters_by_package_ids(package_ids: Vec<String>) -> napi::Result<()> {
    try_join!(
        PrService::delete_by_package_ids(package_ids.clone()),
        RkService::delete_by_package_ids(package_ids.clone()),
        SrkService::delete_by_package_ids(package_ids.clone()),
        NrtlService::delete_by_package_ids(package_ids.clone()),
        NrtlRkService::delete_by_package_ids(package_ids.clone()),
        WilsonService::delete_by_package_ids(package_ids.clone()),
        UniquacService::delete_by_package_ids(package_ids.clone()),
        PsrkService::delete_by_package_ids(package_ids),
    )
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(())
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn delete_binary_parameters_by_ids_no_default(
    ids: Vec<String>,
    func_code: String,
) -> napi::Result<bool> {
    let rows_affected = match func_code.to_uppercase().as_str() {
        "PR" => PrService::delete_by_ids_and_func_code_no_default(ids).await,
        "RK" => RkService::delete_by_ids_and_func_code_no_default(ids).await,
        "SRK" => SrkService::delete_by_ids_and_func_code_no_default(ids).await,
        "NRTL" => NrtlService::delete_by_ids_and_func_code_no_default(ids).await,
        "NRTL-RK" => NrtlRkService::delete_by_ids_and_func_code_no_default(ids).await,
        "WILSON" => WilsonService::delete_by_ids_and_func_code_no_default(ids).await,
        "UNIQUAC" => UniquacService::delete_by_ids_and_func_code_no_default(ids).await,
        "PSRK" => PsrkService::delete_by_ids_and_func_code_no_default(ids).await,

        _ => return Err(napi::Error::from_reason("Unknown funcCode")),
    }
    .map_err(handle_db_err)?;

    Ok(rows_affected)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn delete_binary_parameters_by_ids_has_default(
    ids: Vec<String>,
    func_code: String,
) -> napi::Result<bool> {
    let rows_affected = match func_code.to_uppercase().as_str() {
        "PR" => PrService::delete_by_ids_and_func_code_has_default(ids).await,
        "RK" => RkService::delete_by_ids_and_func_code_has_default(ids).await,
        "SRK" => SrkService::delete_by_ids_and_func_code_has_default(ids).await,
        "NRTL" => NrtlService::delete_by_ids_and_func_code_has_default(ids).await,
        "NRTL-RK" => NrtlRkService::delete_by_ids_and_func_code_has_default(ids).await,
        "WILSON" => WilsonService::delete_by_ids_and_func_code_has_default(ids).await,
        "UNIQUAC" => UniquacService::delete_by_ids_and_func_code_has_default(ids).await,
        "PSRK" => PsrkService::delete_by_ids_and_func_code_has_default(ids).await,

        _ => return Err(napi::Error::from_reason("Unknown funcCode")),
    }
    .map_err(handle_db_err)?;
    Ok(rows_affected)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn get_all_binary_parameter_by_package_ids_default_api(
    func_code: String,
    fluid_package_ids: Vec<String>,
    is_default: u32,
) -> napi::Result<String> {
    // 根据 func_code 获取数据并序列化为 JSON 字符串返回给 Node.js
    let result = match func_code.to_uppercase().as_str() {
        "PR" => serde_json::to_string(
            &PrService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "RK" => serde_json::to_string(
            &RkService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "SRK" => serde_json::to_string(
            &SrkService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "NRTL" => serde_json::to_string(
            &NrtlService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "NRTL-RK" => serde_json::to_string(
            &NrtlRkService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "WILSON" => serde_json::to_string(
            &WilsonService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "UNIQUAC" => serde_json::to_string(
            &UniquacService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        "PSRK" => serde_json::to_string(
            &PsrkService::get_all_binary_parameter_by_package_ids_default(
                fluid_package_ids,
                is_default,
            )
            .await
            .map_err(handle_db_err)?,
        ),
        _ => return Err(napi::Error::from_reason("Unknown funcCode")),
    }
    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(result)
}
#[napi(namespace = "modelFluidPackageBinary")]
pub async fn get_all_binary_message_by_package_id_is_default(
    package_id: String,
    is_default: u32,
) -> Result<Vec<BinaryAllMessageResultDTO>> {
    let mut results = Vec::new();
    let codes = vec![
        "PR", "RK", "SRK", "NRTL", "NRTL-RK", "WILSON", "UNIQUAC", "PSRK",
    ];

    // 辅助函数：处理解析
    let to_f64 = |s: String| s.parse::<f64>().unwrap_or(0.0);
    for code in codes {
        // 重要：这里不再让 match 返回 list，而是直接返回最终的 serde_json::Value
        let datas: serde_json::Value = match code {
            "PR" => {
                let list =
                    PrService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                              "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "RK" => {
                let list =
                    RkService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                             "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "SRK" => {
                let list =
                    SrkService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                             "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "NRTL" => {
                let list =
                    NrtlService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "FIJ": to_f64(item.fij), "FJI": to_f64(item.fji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            "NRTL-RK" => {
                let list =
                    NrtlRkService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "FIJ": to_f64(item.fij), "FJI": to_f64(item.fji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            "WILSON" => {
                let list =
                    WilsonService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }
            "UNIQUAC" => {
                let list =
                    UniquacService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }
            "PSRK" => {
                let list =
                    PsrkService::find_by_package_id_and_default(package_id.clone(), is_default)
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "TIJ": to_f64(item.tij), "TJI": to_f64(item.tji),
                                "VIJ": to_f64(item.vij), "VJI": to_f64(item.vji),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            _ => serde_json::Value::Array(vec![]),
        };

        results.push(BinaryAllMessageResultDTO {
            func_code: code.to_string(),
            datas,
        });
    }

    Ok(results)
}

#[napi(
    object,
    namespace = "modelFluidPackageBinary",
    js_name = "BinaryAllMessageResultDTO"
)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryAllMessageResultDTO {
    pub func_code: String,
    pub datas: serde_json::Value,
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn get_all_binary_message_by_package_ids(
    package_ids: Vec<String>,
) -> Result<Vec<BinaryAllMessageResultDTO>> {
    let mut results = Vec::new();
    let codes = vec![
        "PR", "RK", "SRK", "NRTL", "NRTL-RK", "WILSON", "UNIQUAC", "PSRK",
    ];

    // 辅助函数：处理解析
    let to_f64 = |s: String| s.parse::<f64>().unwrap_or(0.0);
    for code in codes {
        // 重要：这里不再让 match 返回 list，而是直接返回最终的 serde_json::Value
        let datas: serde_json::Value = match code {
            "PR" => {
                let list = PrService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                              "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "RK" => {
                let list = RkService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                             "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "SRK" => {
                let list = SrkService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                             "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "NRTL" => {
                let list =
                    NrtlService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "FIJ": to_f64(item.fij), "FJI": to_f64(item.fji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            "NRTL-RK" => {
                let list =
                    NrtlRkService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "FIJ": to_f64(item.fij), "FJI": to_f64(item.fji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            "WILSON" => {
                let list =
                    WilsonService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }
            "UNIQUAC" => {
                let list =
                    UniquacService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }
            "PSRK" => {
                let list =
                    PsrkService::get_all_binary_parameter_by_package_ids(package_ids.clone())
                        .await
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "TIJ": to_f64(item.tij), "TJI": to_f64(item.tji),
                                "VIJ": to_f64(item.vij), "VJI": to_f64(item.vji),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            _ => serde_json::Value::Array(vec![]),
        };

        results.push(BinaryAllMessageResultDTO {
            func_code: code.to_string(),
            datas,
        });
    }

    Ok(results)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn get_all_binary_message_by_package_ids_default(
    package_ids: Vec<String>,
    is_default: u32,
) -> Result<Vec<BinaryAllMessageResultDTO>> {
    let mut results = Vec::new();
    let codes = vec![
        "PR", "RK", "SRK", "NRTL", "NRTL-RK", "WILSON", "UNIQUAC", "PSRK",
    ];

    // 辅助函数：处理解析
    let to_f64 = |s: String| s.parse::<f64>().unwrap_or(0.0);
    for code in codes {
        // 重要：这里不再让 match 返回 list，而是直接返回最终的 serde_json::Value
        let datas: serde_json::Value = match code {
            "PR" => {
                let list = PrService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                              "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "RK" => {
                let list = RkService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                             "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "SRK" => {
                let list = SrkService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(list.into_iter().map(|item| {
                        serde_json::json!({
                            "KAIJ": to_f64(item.kaij), "KBIJ": to_f64(item.kbij), "KCIJ": to_f64(item.kcij),
                            "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                            "componentI": item.compound_i, "componentJ": item.compound_j,
                             "fluidPackageId": item.fluid_package_id,
                        })
                    }).collect::<Vec<_>>())
            }
            "NRTL" => {
                let list = NrtlService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "FIJ": to_f64(item.fij), "FJI": to_f64(item.fji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            "NRTL-RK" => {
                let list = NrtlRkService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "FIJ": to_f64(item.fij), "FJI": to_f64(item.fji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            "WILSON" => {
                let list = WilsonService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }
            "UNIQUAC" => {
                let list = UniquacService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "AIJ": to_f64(item.aij), "AJI": to_f64(item.aji),
                                "BIJ": to_f64(item.bij), "BJI": to_f64(item.bji),
                                "CIJ": to_f64(item.cij), "DIJ": to_f64(item.dij),
                                "EIJ": to_f64(item.eij), "EJI": to_f64(item.eji),
                                "minT": to_f64(item.min_t), "maxT": to_f64(item.max_t),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }
            "PSRK" => {
                let list = PsrkService::get_all_binary_parameter_by_package_ids_default(
                    package_ids.clone(),
                    is_default,
                )
                .await
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
                serde_json::json!(
                    list.into_iter()
                        .map(|item| {
                            serde_json::json!({
                                "TIJ": to_f64(item.tij), "TJI": to_f64(item.tji),
                                "VIJ": to_f64(item.vij), "VJI": to_f64(item.vji),
                                "componentI": item.compound_i, "componentJ": item.compound_j,
                                 "fluidPackageId": item.fluid_package_id,
                            })
                        })
                        .collect::<Vec<_>>()
                )
            }

            _ => serde_json::Value::Array(vec![]),
        };

        results.push(BinaryAllMessageResultDTO {
            func_code: code.to_string(),
            datas,
        });
    }

    Ok(results)
}

#[napi(namespace = "modelFluidPackageBinary")]
pub async fn delete_binary_interaction_by_cas(
    fluid_package_id: String,
    cas_nos: Vec<String>,
) -> napi::Result<()> {
    // 依次删除所有表中的相关数据
    let _ = PrService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
        .await;
    let _ = RkService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
        .await;
    let _ = SrkService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
        .await;
    let _ =
        NrtlService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
            .await;
    let _ =
        NrtlRkService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
            .await;
    let _ =
        WilsonService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
            .await;
    let _ =
        UniquacService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
            .await;
    let _ =
        PsrkService::delete_by_package_id_and_cas_nos(fluid_package_id.clone(), cas_nos.clone())
            .await;

    Ok(())
}
