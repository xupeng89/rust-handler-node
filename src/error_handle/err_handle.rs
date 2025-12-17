/// 数据库异常处理
use napi::*;
use sea_orm::DbErr;
use std::fmt;
#[derive(Debug)]
pub enum SyncError {
    CacheReadError(sea_orm::DbErr),  // 缓存数据库读取失败
    LocalWriteError(sea_orm::DbErr), // 本地数据库写入失败
    LocalReadError(sea_orm::DbErr),  // 本地数据库读取失败
    CacheWriteError(sea_orm::DbErr), // 缓存数据库写入失败
    NoDataToSync,                    // 无数据可同步（边界处理）
}

// 实现错误转换（适配 napi::Error）
impl From<SyncError> for napi::Error {
    fn from(err: SyncError) -> Self {
        let msg = match err {
            SyncError::CacheReadError(e) => format!("缓存数据库读取失败：{}", e),
            SyncError::LocalWriteError(e) => format!("本地数据库写入失败：{}", e),
            SyncError::LocalReadError(e) => format!("本地数据库读取失败：{}", e),
            SyncError::CacheWriteError(e) => format!("缓存数据库写入失败：{}", e),
            SyncError::NoDataToSync => "无数据可同步".to_string(),
        };
        napi::Error::new(napi::Status::Unknown, msg)
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::CacheReadError(e) => write!(f, "缓存读取失败：{}", e),
            SyncError::LocalWriteError(e) => write!(f, "本地写入失败：{}", e),
            SyncError::LocalReadError(e) => write!(f, "本地读取失败：{}", e),
            SyncError::CacheWriteError(e) => write!(f, "缓存写入失败：{}", e),
            SyncError::NoDataToSync => write!(f, "无数据可同步"),
        }
    }
}

/// 将 sea_orm::DbErr 转换为 napi::Error，以便在 JS 中抛出异常
pub fn handle_db_err(e: DbErr) -> Error {
    // 简化错误处理，使用 Debug 格式输出 DbErr
    Error::new(Status::GenericFailure, format!("Database Error: {:?}", e))
}
