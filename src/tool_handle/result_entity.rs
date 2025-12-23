use napi_derive::napi;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
/// 页面使用的label, value 返回
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
#[napi(object, namespace = "resultOption")]
pub struct FunctionOptionDto {
    pub label: String,
    pub value: String,
}
