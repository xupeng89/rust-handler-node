use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_util_handle::model_trend_chart_config_entity::{
    ActiveModel as ConfigActiveModel, Column as ConfigColumn, Entity as ConfigEntity,
    Model as ConfigModel,
};
use napi_derive::napi;
use sea_orm::{NotSet,FromQueryResult, QueryFilter, Set, entity::prelude::*, PaginatorTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelTrendChart",
    js_name = "ModelTrendChartConfigDTO"
)]
pub struct ModelTrendChartConfigDTO {
    pub id: i32,
    pub model_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelTrendChart",
    js_name = "ModelTrendChartConfigUpdateDTO"
)]
pub struct ModelTrendChartConfigUpdateDTO {
    pub id: i32,
    pub model_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

// 转换逻辑
impl From<ConfigModel> for ModelTrendChartConfigDTO {
    fn from(m: ConfigModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            name: m.name,
            description: m.description,
        }
    }
}

impl ModelTrendChartConfigDTO {
    fn into_active_model(self) -> ConfigActiveModel {
        ConfigActiveModel {
            // id 是自增的，插入时通常设为 NotSet 或由外部传入
            id: if self.id == 0 { NotSet } else { Set(self.id) },
            model_id: Set(self.model_id),
            name: Set(self.name),
            description: Set(self.description),
        }
    }
}

/// 插入实体
pub async fn insert(data: ModelTrendChartConfigDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let am = data.into_active_model();
    ConfigEntity::insert(am).exec(db).await?;
    Ok(true)
}

/// 根据 model_id 和 name 获取数量
pub async fn get_count_by_name(model_id: String, name: String) -> Result<i64, DbErr> {
    let db = get_business_db().await?;
    let count = ConfigEntity::find()
        .filter(ConfigColumn::ModelId.eq(model_id))
        .filter(ConfigColumn::Name.eq(name))
        .count(db)
        .await?;
    Ok(count as i64)
}

/// 获取列表
pub async fn get_many_by_model_id(
    model_id: String,
) -> Result<Vec<ModelTrendChartConfigDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ConfigEntity::find()
        .filter(ConfigColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelTrendChartConfigDTO::from)
        .collect())
}

/// 批量删除 (限定 model_id)
pub async fn delete_by_ids(model_id: String, ids: Vec<i32>) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = ConfigEntity::delete_many()
        .filter(ConfigColumn::ModelId.eq(model_id))
        .filter(ConfigColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}
