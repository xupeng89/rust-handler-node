use crate::service_database::database_config::db_config_connection::get_config_db;
use napi_derive::napi;

use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, Set};

use serde::{Deserialize, Serialize};
// ======================================
use crate::service_database::database_config::entity::conf_config_entity::{
    ActiveModel as ConfConfigActiveModel, ConfConfigValueTypeEnum, Entity as ConfConfigEntity,
    Model as ConfConfigModel,
};

// 针对 NAPI 调用的 DTO (Data Transfer Object)
// 字段与 Model 一致，但添加 napi(object) 宏
#[napi(object, namespace = "confConfig")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDto {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub value: String,
    pub value_type: ConfConfigValueTypeEnum,
}

// 辅助函数：将 Model 转换为 ConfigDto
impl From<ConfConfigModel> for ConfigDto {
    fn from(model: ConfConfigModel) -> Self {
        ConfigDto {
            id: model.id,
            name: model.name,
            code: model.code,
            value: model.value,
            value_type: model.value_type,
        }
    }
}

pub async fn select_all_conf_config() -> Result<Vec<ConfigDto>, DbErr> {
    let db = get_config_db().await.unwrap();

    // 直接查询 ID 为 1 的记录
    let model = ConfConfigEntity::find().all(db).await?;

    // 将结果转换为 ConfigDto
    let dto = model.into_iter().map(ConfigDto::from).collect::<Vec<_>>();

    Ok(dto)
}
pub async fn upsert_and_insert_all_conf_config(
    config_data_list: Vec<ConfigDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap();

    // 1. 尝试查找 ID = 1 的现有记录
    let existing_models = ConfConfigEntity::find().all(db).await?;

    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    // 根据code,本身数据库有数据的进行更新，没有的进行插入
    for config in config_data_list {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfConfigActiveModel = model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.value = Set(config.value);
            active_model.value_type = Set(config.value_type);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfConfigActiveModel {
                id: NotSet,
                name: Set(config.name),
                code: Set(config.code),
                value: Set(config.value),
                value_type: Set(config.value_type),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}
