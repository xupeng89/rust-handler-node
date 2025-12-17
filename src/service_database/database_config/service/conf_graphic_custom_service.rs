use crate::service_database::database_config::db_config_connection::get_config_db;
use napi_derive::napi;

use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, Set};

use serde::{Deserialize, Serialize};
// ======================================
use crate::service_database::database_config::entity::conf_graphic_custom_entity::{
    ActiveModel as ConfGraphicCustomActiveModel, Entity as ConfGraphicCustomEntity,
    Model as ConfGraphicCustomModel,
};
// 针对 NAPI 调用的 DTO (Data Transfer Object)
// 字段与 Model 一致，但添加 napi(object) 宏
#[napi(object, namespace = "confGraphicCustom")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfGraphicCustomDto {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub custom_type: String,
    pub arithmetic: String,
    pub size: String,
    pub svg: String,
    pub ports: String,
    pub window_size: String,
    pub dnd_type: String,
}

// 辅助函数：将 Model 转换为 ConfigDto
impl From<ConfGraphicCustomModel> for ConfGraphicCustomDto {
    fn from(model: ConfGraphicCustomModel) -> Self {
        ConfGraphicCustomDto {
            id: model.id,
            name: model.name,
            code: model.code,
            custom_type: model.custom_type,
            arithmetic: model.arithmetic,
            size: model.size,
            svg: model.svg,
            ports: model.ports,
            window_size: model.window_size,
            dnd_type: model.dnd_type,
        }
    }
}

pub async fn select_all_conf_graphic_custom() -> Result<Vec<ConfGraphicCustomDto>, DbErr> {
    let db = get_config_db().await.unwrap();

    // 直接查询 ID 为 1 的记录
    let model = ConfGraphicCustomEntity::find().all(db).await?;

    // 将结果转换为 ConfigDto
    let dto = model
        .into_iter()
        .map(ConfGraphicCustomDto::from)
        .collect::<Vec<_>>();

    Ok(dto)
}
pub async fn upsert_and_insert_all_conf_graphic_custom(
    config_data_list: Vec<ConfGraphicCustomDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap();

    // 1. 尝试查找 ID = 1 的现有记录
    let existing_models = ConfGraphicCustomEntity::find().all(db).await?;

    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    // 根据code,本身数据库有数据的进行更新，没有的进行插入
    for config in config_data_list {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfGraphicCustomActiveModel = model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.custom_type = Set(config.custom_type);
            active_model.arithmetic = Set(config.arithmetic);
            active_model.size = Set(config.size);
            active_model.svg = Set(config.svg);
            active_model.ports = Set(config.ports);
            active_model.window_size = Set(config.window_size);
            active_model.dnd_type = Set(config.dnd_type);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfGraphicCustomActiveModel {
                id: NotSet,
                name: Set(config.name),
                custom_type: Set(config.custom_type),
                arithmetic: Set(config.arithmetic),
                size: Set(config.size),
                svg: Set(config.svg),
                ports: Set(config.ports),
                window_size: Set(config.window_size),
                dnd_type: Set(config.dnd_type),
                code: Set(config.code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}
