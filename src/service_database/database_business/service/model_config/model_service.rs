use crate::service_database::database_business::db_business_connection::get_business_db; // 替换为你实际的连接获取路径
use crate::service_database::database_business::entity::model_config::model_entity::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{FromQueryResult, QueryFilter, QueryOrder, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::tool_handle::time_tool::naive_dt_utc_to_millis;

// ======================================
// DTO 定义
// ======================================
#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(object, namespace = "modelHandle", js_name = "ModelDTO")]
pub struct ModelDTO {
    pub id: String,
    pub model_no: String,
    pub developer: String,
    pub model_name: String,
    pub update_at: String,
    pub create_at: String,
    pub status: i32,
    pub standard_temperature: f64,
    pub standard_pressure: f64,
    pub grid_state: String,
    pub grid_color: String,
    pub grid_size: i32,
    pub standard_temperature_unit: String,
    pub standard_pressure_unit: String,
    pub default_model_id: String,
}

impl From<ModelModel> for ModelDTO {
    fn from(m: ModelModel) -> Self {
        ModelDTO {
            id: m.id,
            model_no: m.model_no,
            developer: m.developer,
            model_name: m.model_name,
            update_at: naive_dt_utc_to_millis(m.update_at),
            create_at: naive_dt_utc_to_millis(m.create_at),
            status: m.status,
            standard_temperature: m.standard_temperature,
            standard_pressure: m.standard_pressure,
            grid_state: m.grid_state,
            grid_color: m.grid_color,
            grid_size: m.grid_size,
            standard_temperature_unit: m.standard_temperature_unit,
            standard_pressure_unit: m.standard_pressure_unit,
            default_model_id: m.default_model_id,
        }
    }
}

impl ModelDTO {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            model_no: Set(self.model_no),
            model_name: Set(self.model_name),
            developer: Set(self.developer),
            standard_temperature: Set(self.standard_temperature),
            standard_temperature_unit: Set(self.standard_temperature_unit),
            standard_pressure: Set(self.standard_pressure),
            standard_pressure_unit: Set(self.standard_pressure_unit),
            grid_state: Set(self.grid_state),
            grid_color: Set(self.grid_color),
            grid_size: Set(self.grid_size),
            default_model_id: Set(self.default_model_id),
            // 时间通常由服务端生成
            create_at: Set(chrono::Utc::now().naive_utc()),
            update_at: Set(chrono::Utc::now().naive_utc()),
            status: Set(0), // 默认正常状态
        }
    }
}

/// 根据 ID 获取一条记录
pub async fn get_model_by_id(id: String) -> Result<ModelDTO, DbErr> {
    let db = get_business_db().await?;
    let model = ModelEntity::find_by_id(id).one(db).await?;
    Ok(model.map(ModelDTO::from).unwrap())
}

/// 根据 ID 检查是否存在
pub async fn has_model_by_id(id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let count = ModelEntity::find()
        .filter(ModelColumn::Id.eq(id))
        .count(db)
        .await?;
    Ok(count > 0)
}

/// 获取最近更新的一条
pub async fn get_first_model_by_update_time() -> Result<ModelDTO, DbErr> {
    let db = get_business_db().await?;
    let model = ModelEntity::find()
        .order_by_desc(ModelColumn::UpdateAt)
        .one(db)
        .await?;
    Ok(model.map(ModelDTO::from).unwrap())
}

/// 获取所有状态为 0 的数据
pub async fn get_all_model() -> Result<Vec<ModelDTO>, DbErr> {
    let db = get_business_db().await?;
    let model = ModelEntity::find()
        .filter(ModelColumn::Status.eq(0))
        .order_by_desc(ModelColumn::UpdateAt)
        .all(db)
        .await?;
    Ok(model.into_iter().map(ModelDTO::from).collect())
}

/// 插入单条数据
pub async fn insert_model(model_data: ModelDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;

    let active_model: ActiveModel = model_data.into_active_model();
    let result = active_model.insert(db).await?;
    Ok(result.id)
}

/// 根据 ModelNo 查询重复，返回 Option<ID>
pub async fn select_model_by_no(model_no: String) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelNo.eq(model_no))
        .one(db)
        .await?;
    Ok(res.map(|m| m.id).unwrap())
}

pub async fn select_model_by_name(name: String) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelName.eq(name))
        .one(db)
        .await?;
    Ok(res.map(|m| m.id).unwrap())
}

/// 逻辑删除
pub async fn delete_model_by_id(id: String) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::update_many()
        .col_expr(ModelColumn::Status, Expr::value(1))
        .filter(ModelColumn::Id.eq(id))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

#[napi(object, namespace = "modelHandle", js_name = "ModelUpdateDTO")]
pub struct ModelUpdateDTO {
    pub id: String, // ID 是必传的
    pub model_no: Option<String>,
    pub developer: Option<String>,
    pub model_name: Option<String>,
    pub standard_temperature: Option<f64>,
    pub standard_pressure: Option<f64>,
    pub grid_state: Option<String>,
    pub grid_color: Option<String>,
    pub grid_size: Option<i32>,
    pub standard_temperature_unit: Option<String>,
    pub standard_pressure_unit: Option<String>,
    pub default_model_id: Option<String>,
    // 状态也可以可选
    pub status: Option<i32>,
}

/// 更新数据 (Partial Update 逻辑)
pub async fn update_model(model_data: ModelUpdateDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    // 1. 先从数据库查找现有记录
    // 1. 获取现有记录
    let existing_model = ModelEntity::find_by_id(model_data.id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Model not found".to_owned()))?;

    // 2. 转换为 ActiveModel
    let mut active_model: ActiveModel = existing_model.into();

    // 3. 有条件地设置字段 (只有当 Option 为 Some 时才更新)
    if let Some(val) = model_data.model_no {
        active_model.model_no = Set(val);
    }
    if let Some(val) = model_data.model_name {
        active_model.model_name = Set(val);
    }
    if let Some(val) = model_data.developer {
        active_model.developer = Set(val);
    }
    if let Some(val) = model_data.standard_temperature {
        active_model.standard_temperature = Set(val);
    }
    if let Some(val) = model_data.standard_temperature_unit {
        active_model.standard_temperature_unit = Set(val);
    }
    if let Some(val) = model_data.standard_pressure {
        active_model.standard_pressure = Set(val);
    }
    if let Some(val) = model_data.standard_pressure_unit {
        active_model.standard_pressure_unit = Set(val);
    }
    if let Some(val) = model_data.grid_state {
        active_model.grid_state = Set(val);
    }
    if let Some(val) = model_data.grid_color {
        active_model.grid_color = Set(val);
    }
    if let Some(val) = model_data.grid_size {
        active_model.grid_size = Set(val);
    }
    if let Some(val) = model_data.default_model_id {
        active_model.default_model_id = Set(val);
    }
    if let Some(val) = model_data.status {
        active_model.status = Set(val);
    }

    // 强制更新时间
    active_model.update_at = Set(chrono::Utc::now().naive_utc());

    // 4. 执行更新
    // SeaORM 会自动对比哪些字段被 Set 过了，生成的 SQL 仅包含改变的部分
    let updated_model = active_model.update(db).await?;

    Ok(updated_model.id)
}
