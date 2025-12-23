use napi_derive::napi;
use serde::{Deserialize, Serialize};
// 映射二元交互参数
#[napi(string_enum)]
#[derive(Debug, Serialize, Deserialize)]
pub enum BinaryFuncCode {
    PR,
    RK,
    SRK,
    NRTL,
    NRTLRK,
    WILSON,
    UNIQUAC,
    PSRK,
}
