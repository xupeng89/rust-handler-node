use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_config_entity::{
    ActiveModel as ModelConfigActiveModel, Column as ModelConfigColumn,
    Entity as ModelConfigEntity, Model as ModelConfigModel,
};
use napi_derive::napi;
use sea_orm::QuerySelect;
use sea_orm::{entity::prelude::*, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[napi(object, namespace = "modelConfig", js_name = "AutoShutterParams")]
#[derive(Serialize, Deserialize, Default)]
pub struct AutoShutterParams {
    pub auto_shutter: i32,
    pub auto_time_interval: i32,
    pub auto_count: i32,
}

#[napi(object, namespace = "modelConfig", js_name = "ModelConfigDTO")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelConfigDTO {
    pub id: i32,
    pub property_params: String,
    pub control_params: String,
    pub rate_params: String,
    pub flash_params: String,
    pub model_id: String,
    pub model_state: i32,
    pub filter_label_params: String,
    pub range_status: i32,
    pub show_label_params: String,
    pub auto_shutter_params: String,
    pub oil_params: String,
    pub relate_point: String,
    pub relate_interlock: String,
    pub default_params: String,
}

impl From<ModelConfigModel> for ModelConfigDTO {
    fn from(c: ModelConfigModel) -> Self {
        ModelConfigDTO {
            id: c.id,
            model_id: c.model_id,
            property_params: c.property_params,
            control_params: c.control_params,
            rate_params: c.rate_params,
            flash_params: c.flash_params,
            model_state: c.model_state,
            filter_label_params: c.filter_label_params,
            range_status: c.range_status,
            show_label_params: c.show_label_params,
            auto_shutter_params: c.auto_shutter_params,
            oil_params: c.oil_params,
            relate_point: c.relate_point,
            relate_interlock: c.relate_interlock,
            default_params: c.default_params,
        }
    }
}
impl From<ModelConfigDTO> for ModelConfigActiveModel {
    fn from(data: ModelConfigDTO) -> Self {
        Self {
            property_params: Set(data.property_params),
            control_params: Set(data.control_params),
            rate_params: Set(data.rate_params),
            flash_params: Set(data.flash_params),
            model_id: Set(data.model_id),
            model_state: Set(data.model_state),
            filter_label_params: Set(data.filter_label_params),
            range_status: Set(data.range_status),
            show_label_params: Set(data.show_label_params),
            auto_shutter_params: Set(data.auto_shutter_params),
            oil_params: Set(data.oil_params),
            relate_point: Set(data.relate_point),
            relate_interlock: Set(data.relate_interlock),
            default_params: Set(data.default_params),
            ..Default::default() // 处理 id 等自增主键
        }
    }
}

#[napi(object, namespace = "modelConfig", js_name = "ModelConfigUpdateDTO")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelConfigUpdateDTO {
    pub id: Option<i32>,
    pub property_params: Option<String>,
    pub control_params: Option<String>,
    pub rate_params: Option<String>,
    pub flash_params: Option<String>,
    pub model_id: String,
    pub model_state: Option<i32>,
    pub filter_label_params: Option<String>,
    pub range_status: Option<i32>,
    pub show_label_params: Option<String>,
    pub auto_shutter_params: Option<String>,
    pub oil_params: Option<String>,
    pub relate_point: Option<String>,
    pub relate_interlock: Option<String>,
    pub default_params: Option<String>,
}

pub async fn get_model_config_detail_by_model_id(
    model_id: String,
) -> Result<ModelConfigDTO, DbErr> {
    let db = get_business_db().await?;

    let result: Option<ModelConfigModel> = ModelConfigEntity::find()
        .filter(ModelConfigColumn::ModelId.eq(model_id))
        .one(db)
        .await?;

    Ok(result.map(ModelConfigDTO::from).unwrap())
}

pub async fn update_model_config_detail_by_model_id(
    model_id: String,
    data: ModelConfigUpdateDTO,
) -> Result<String, DbErr> {
    let db = get_business_db().await?;

    // 1. 查找现有记录
    let existing = ModelConfigEntity::find()
        .filter(ModelConfigColumn::ModelId.eq(model_id.clone()))
        .one(db)
        .await?;

    // 2. 转换为 ActiveModel 并进行局部更新 (Partial Update)
    let mut active_model: ModelConfigActiveModel = existing.unwrap().into();

    if let Some(val) = data.property_params {
        active_model.property_params = Set(val);
    }
    if let Some(val) = data.control_params {
        active_model.control_params = Set(val);
    }
    if let Some(val) = data.rate_params {
        active_model.rate_params = Set(val);
    }
    if let Some(val) = data.flash_params {
        active_model.flash_params = Set(val);
    }
    if let Some(val) = data.model_state {
        active_model.model_state = Set(val);
    }
    if let Some(val) = data.filter_label_params {
        active_model.filter_label_params = Set(val);
    }
    if let Some(val) = data.range_status {
        active_model.range_status = Set(val);
    }
    if let Some(val) = data.show_label_params {
        active_model.show_label_params = Set(val);
    }
    if let Some(val) = data.auto_shutter_params {
        active_model.auto_shutter_params = Set(val);
    }
    if let Some(val) = data.oil_params {
        active_model.oil_params = Set(val);
    }
    if let Some(val) = data.relate_point {
        active_model.relate_point = Set(val);
    }
    if let Some(val) = data.relate_interlock {
        active_model.relate_interlock = Set(val);
    }
    if let Some(val) = data.default_params {
        active_model.default_params = Set(val);
    }

    active_model.update(db).await?;

    Ok(model_id)
}

pub async fn insert_model_config_detail(data: ModelConfigDTO) -> Result<i32, DbErr> {
    let db = get_business_db().await?;
    let active_model: ModelConfigActiveModel = data.into();
    let result = active_model.insert(db).await?;
    Ok(result.id)
}

pub async fn get_model_config_auto_shutter_config(
    model_id: String,
) -> Result<AutoShutterParams, DbErr> {
    let db = get_business_db().await?;
    let res = ModelConfigEntity::find()
        .select_only()
        .column(ModelConfigColumn::AutoShutterParams)
        .filter(ModelConfigColumn::ModelId.eq(model_id))
        .into_tuple::<String>()
        .one(db)
        .await?;

    let json_str = res
        .unwrap_or_else(|| r#"{"autoShutter":1,"autoTimeInterval":30,"autoCount":60}"#.to_string());
    let params: AutoShutterParams = serde_json::from_str(&json_str).unwrap_or_default();
    Ok(params)
}

#[napi(object, namespace = "modelConfig")]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FilterLabelItem {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub unit_code: String,
    pub decimal: i32,
    pub svg: String,
    pub location: String,
    pub r#type: String, // type 是 Rust 关键字，需用 r#
    pub show: bool,
}

#[napi(object, namespace = "modelConfig")]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ShowLabelParams {
    pub global_label: Vec<FilterLabelItem>,
    pub flow_sheet_label: Vec<FilterLabelItem>,
}

#[napi(object)]
pub struct FilterLabelParamsResult {
    pub filter_label_params: Vec<FilterLabelItem>,
    pub range_status: i32,
    pub show_label_params: ShowLabelParams,
}

pub async fn get_model_config_for_filter_label_params(
    model_id: String,
) -> Result<FilterLabelParamsResult, DbErr> {
    let db = get_business_db().await?;

    // 1. 查询指定的三个字段
    let result = ModelConfigEntity::find()
        .select_only()
        .column(ModelConfigColumn::FilterLabelParams)
        .column(ModelConfigColumn::RangeStatus)
        .column(ModelConfigColumn::ShowLabelParams)
        .filter(ModelConfigColumn::ModelId.eq(model_id))
        .into_tuple::<(String, i32, String)>() // 映射为元组 (filter, range, show)
        .one(db)
        .await?;

    if let Some((filter_str, range_status, show_str)) = result {
        // 2. 解析 JSON 字符串
        // 如果解析失败，返回默认值而不是让整个接口报错
        let filter_label_params: Vec<FilterLabelItem> =
            serde_json::from_str(&filter_str).unwrap_or_else(|_| vec![]);

        let show_label_params: ShowLabelParams =
            serde_json::from_str(&show_str).unwrap_or_else(|_| ShowLabelParams {
                global_label: vec![],
                flow_sheet_label: vec![],
            });

        Ok(FilterLabelParamsResult {
            filter_label_params,
            range_status,
            show_label_params,
        })
    } else {
        Ok(None.unwrap())
    }
}

#[napi(object)]
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ControlParams {
    pub mode: String,
    pub real_flag: i32,
    pub step: String,
    pub acceleration: i32,
    pub current_time: String,
    pub end_step: i32,
    pub real_factor: i32,
    pub display_interval: i32,
    pub sim_time: i32,
    pub real_time: i32,
    pub run_count: i32,
    pub cur_time: i32,
}

#[napi(object)]
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RateParams {
    pub pf_solver: i32,
    pub control_and_logic: i32,
    pub energy_calc: i32,
    pub flash_calc: i32,
}

#[napi(object)]
pub struct ControlAndRateResult {
    pub id: Option<i32>,
    pub control_params: ControlParams,
    pub rate_params: RateParams,
}

pub async fn get_model_config_control_and_rate_params(
    model_id: String,
) -> Result<ControlAndRateResult, DbErr> {
    let db = get_business_db().await?;

    // 1. 查询 id, control_params, rate_params
    let result = ModelConfigEntity::find()
        .select_only()
        .column(ModelConfigColumn::Id)
        .column(ModelConfigColumn::ControlParams)
        .column(ModelConfigColumn::RateParams)
        .filter(ModelConfigColumn::ModelId.eq(model_id))
        .into_tuple::<(i32, String, String)>()
        .one(db)
        .await?;

    if let Some((id, control_str, rate_str)) = result {
        // 2. 解析 JSON (如果解析失败则使用 Default)
        let control_params: ControlParams = serde_json::from_str(&control_str).unwrap_or_default();

        let rate_params: RateParams = serde_json::from_str(&rate_str).unwrap_or_default();

        Ok(ControlAndRateResult {
            id: Some(id),
            control_params,
            rate_params,
        })
    } else {
        Ok(None.unwrap())
    }
}
