use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_file_comparison_entity::{
    ActiveModel as FileCompActiveModel, Column as FileCompColumn, Entity as FileCompEntity,
    Model as FileCompModel,
};
use sea_orm::{ActiveModelTrait, QueryFilter, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use napi_derive::napi;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "modelFile", js_name = "ModelFileComparisonDTO")]
pub struct ModelFileComparisonDTO {
    pub id: Option<i32>,
    pub code: String, // 对应 TS 的 md5Code
    pub file_name: String,
}

impl From<FileCompModel> for ModelFileComparisonDTO {
    fn from(m: FileCompModel) -> Self {
        Self {
            id: Some(m.id),
            code: m.code,
            file_name: m.file_name,
        }
    }
}

pub struct ModelFileComparisonService;

impl ModelFileComparisonService {
    /// 查找 MD5 Code (对应 getModelFileComparisonOneByfileName)
    pub async fn get_code_by_file_name(file_name: String) -> Result<String, DbErr> {
        let db = get_business_db().await?;
        let res = FileCompEntity::find()
            .filter(FileCompColumn::FileName.eq(file_name))
            .one(db)
            .await?;

        Ok(res.map(|m| m.code).unwrap_or_default())
    }

    /// 插入或更新 (对应 insertOrUpdateModelFileComparison)
    pub async fn insert_or_update(code: String, file_name: String) -> Result<(), DbErr> {
        let db = get_business_db().await?;

        let existing = FileCompEntity::find()
            .filter(FileCompColumn::FileName.eq(file_name.clone()))
            .one(db)
            .await?;

        if let Some(model) = existing {
            // 更新
            let mut am: FileCompActiveModel = model.into();
            am.code = Set(code);
            am.update(db).await?;
        } else {
            // 插入
            let am = FileCompActiveModel {
                code: Set(code),
                file_name: Set(file_name),
                ..Default::default() // id 是自增的主键，不设置
            };
            am.insert(db).await?;
        }
        Ok(())
    }

    /// 根据文件名删除
    pub async fn delete_by_file_name(file_name: String) -> Result<(), DbErr> {
        let db = get_business_db().await?;
        FileCompEntity::delete_many()
            .filter(FileCompColumn::FileName.eq(file_name))
            .exec(db)
            .await?;
        Ok(())
    }
}
