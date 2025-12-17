use crate::service_database::database_config::db_config_connection::get_config_db;

use napi_derive::napi;

use crate::tool_handle::time_tool::{millis_to_naive_dt_utc, naive_dt_utc_to_millis};
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, Set};

use serde::{Deserialize, Serialize};
// ======================================
// 假设 ConfFunctionPic 的实体定义在这里
use crate::service_database::database_config::entity::conf_model_entity::{
    ActiveModel as ConfModelActiveModel, Entity as ConfModelEntity, Model as ConfModelModel,
};

#[napi(object, namespace = "confModel")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfModelDto {
    pub create_at: String,
    pub update_at: String,
    pub standard_temperature: f64,
    pub standard_temperature_unit: String,
    pub standard_pressure: f64,
    pub standard_pressure_unit: String,
    pub grid_size: String,
    pub grid_color: String,
    pub grid_state: String,
    pub language: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfModelModel> for ConfModelDto {
    fn from(model: ConfModelModel) -> Self {
        ConfModelDto {
            create_at: naive_dt_utc_to_millis(model.create_at),
            update_at: naive_dt_utc_to_millis(model.update_at),
            standard_temperature: model.standard_temperature,
            standard_temperature_unit: model.standard_temperature_unit,
            standard_pressure: model.standard_pressure,
            standard_pressure_unit: model.standard_pressure_unit,
            grid_size: model.grid_size,
            grid_color: model.grid_color,
            grid_state: model.grid_state,
            language: model.language,
        }
    }
}

pub async fn select_conf_model_one() -> Result<ConfModelDto, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let model = ConfModelEntity::find().one(db).await?;

    // 将查询结果 (Option<ConfModelModel>) 转换为 Option<ConfModelDto>
    let dto: ConfModelDto = model.map(ConfModelDto::from).unwrap();

    Ok(dto)
}
pub async fn upsert_and_insert_fixed_conf_pic(data: ConfModelDto) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let model_one = ConfModelEntity::find().one(db).await?;

    if let Some(model) = model_one {
        // 更新逻辑
        let mut active_model: ConfModelActiveModel = model.clone().into_active_model();
        active_model.standard_temperature = Set(data.standard_temperature);
        active_model.standard_temperature_unit = Set(data.standard_temperature_unit);
        active_model.standard_pressure = Set(data.standard_pressure);
        active_model.standard_pressure_unit = Set(data.standard_pressure_unit);
        active_model.grid_size = Set(data.grid_size);
        active_model.grid_color = Set(data.grid_color);
        active_model.grid_state = Set(data.grid_state);
        active_model.language = Set(data.language);
        active_model.update_at = Set(millis_to_naive_dt_utc(data.update_at.parse().unwrap()));
        active_model.update(db).await?;
    } else {
        // 插入逻辑
        let active_model = ConfModelActiveModel {
            id: NotSet,
            standard_temperature: Set(data.standard_temperature),
            standard_temperature_unit: Set(data.standard_temperature_unit),
            standard_pressure: Set(data.standard_pressure),
            standard_pressure_unit: Set(data.standard_pressure_unit),
            grid_size: Set(data.grid_size),
            grid_color: Set(data.grid_color),
            grid_state: Set(data.grid_state),
            language: Set(data.language),
            create_at: Set(millis_to_naive_dt_utc(data.create_at.parse().unwrap())),
            update_at: Set(millis_to_naive_dt_utc(data.update_at.parse().unwrap())),
        };
        active_model.insert(db).await?;
    }
    Ok(1)
}
