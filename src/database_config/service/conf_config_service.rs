use crate::database_config::db_config_connection::get_config_db;
use napi_derive::napi;

use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, Set};

use serde::{Deserialize, Serialize};
// ======================================
use crate::database_config::entity::conf_config_entity::{
    ActiveModel as ConfConfigActiveModel, Entity as ConfConfigEntity, Model as ConfConfigModel,
};

// 针对 NAPI 调用的 DTO (Data Transfer Object)
// 字段与 Model 一致，但添加 napi(object) 宏
#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDto {
    pub id: i32,
    pub property_params: String,
    pub control_params: String,
    pub rate_params: String,
    pub flash_params: String,
    pub filter_label_params: String,
    pub model_state: i32,
    pub show_label_params: String,
    pub range_status: i32,
    pub auto_shutter_params: String,
    pub oil_params: String,
}

// 辅助函数：将 Model 转换为 ConfigDto
impl From<ConfConfigModel> for ConfigDto {
    fn from(model: ConfConfigModel) -> Self {
        ConfigDto {
            id: model.id,
            property_params: model.property_params,
            control_params: model.control_params,
            rate_params: model.rate_params,
            flash_params: model.flash_params,
            filter_label_params: model.filter_label_params,
            model_state: model.model_state,
            show_label_params: model.show_label_params,
            range_status: model.range_status,
            auto_shutter_params: model.auto_shutter_params,
            oil_params: model.oil_params,
        }
    }
}

pub async fn select_fixed_conf_config() -> Result<ConfigDto, DbErr> {
    let db = get_config_db().await.unwrap();

    // 直接查询 ID 为 1 的记录
    let model = ConfConfigEntity::find_by_id(1).one(db).await?;

    // 将结果转换为 ConfigDto
    let dto = model.map(ConfigDto::from).unwrap();

    Ok(dto)
}
pub async fn upsert_fixed_conf_config(config_data: ConfigDto) -> Result<(), DbErr> {
    let db = get_config_db().await.unwrap();
    const FIXED_ID: i32 = 1;

    // 1. 尝试查找 ID = 1 的现有记录
    let existing_model = ConfConfigEntity::find_by_id(FIXED_ID).one(db).await?;

    // 2. 将 DTO 数据转换为 ActiveModel 的 Set 字段
    let active_model_fields = ConfConfigActiveModel {
        // ID 字段的处理取决于操作类型，下面会单独处理
        id: sea_orm::NotSet,
        property_params: Set(config_data.property_params),
        control_params: Set(config_data.control_params),
        rate_params: Set(config_data.rate_params),
        flash_params: Set(config_data.flash_params),
        filter_label_params: Set(config_data.filter_label_params),
        model_state: Set(config_data.model_state),
        show_label_params: Set(config_data.show_label_params),
        range_status: Set(config_data.range_status),
        auto_shutter_params: Set(config_data.auto_shutter_params),
        oil_params: Set(config_data.oil_params),
    };

    match existing_model {
        Some(model) => {
            // --- 📌 更新 (Update) 逻辑 ---

            // 将现有 Model 转换为 ActiveModel，这会保留现有的 ID
            let mut active_model: ConfConfigActiveModel = model.into_active_model();

            // 复制 ActiveModel 中的 Set 字段
            active_model.property_params = active_model_fields.property_params;
            active_model.control_params = active_model_fields.control_params;
            active_model.rate_params = active_model_fields.rate_params;
            active_model.flash_params = active_model_fields.flash_params;
            active_model.filter_label_params = active_model_fields.filter_label_params;
            active_model.model_state = active_model_fields.model_state;
            active_model.show_label_params = active_model_fields.show_label_params;
            active_model.range_status = active_model_fields.range_status;
            active_model.auto_shutter_params = active_model_fields.auto_shutter_params;
            active_model.oil_params = active_model_fields.oil_params;

            // 执行更新
            active_model.update(db).await?;

            Ok(())
        }
        None => {
            // --- ➕ 插入 (Insert) 逻辑 ---

            // 直接使用包含数据的 ActiveModel，但需要将 ID 明确设置为 FIXED_ID
            let active_model = ConfConfigActiveModel {
                id: Set(FIXED_ID), // 关键：手动设置 ID 为 1
                // 复制 ActiveModel 中的 Set 字段
                property_params: active_model_fields.property_params,
                control_params: active_model_fields.control_params,
                rate_params: active_model_fields.rate_params,
                flash_params: active_model_fields.flash_params,
                filter_label_params: active_model_fields.filter_label_params,
                model_state: active_model_fields.model_state,
                show_label_params: active_model_fields.show_label_params,
                range_status: active_model_fields.range_status,
                auto_shutter_params: active_model_fields.auto_shutter_params,
                oil_params: active_model_fields.oil_params,
            };

            // 执行插入
            active_model.insert(db).await?;

            Ok(())
        }
    }
}
