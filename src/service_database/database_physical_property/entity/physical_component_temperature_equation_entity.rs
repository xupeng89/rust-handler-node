use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pp_component_temperature_equation_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub component_id: i32,

    pub name: String,

    pub unit: String,

    pub code: String,

    pub function_code: i32,

    pub function_name: String,

    pub coefficient_a: f64,
    pub coefficient_b: f64,
    pub coefficient_c: f64,
    pub coefficient_d: f64,
    pub coefficient_e: f64,
    pub coefficient_f: f64,
    pub coefficient_g: f64,
    pub coefficient_h: f64,
    pub coefficient_k: f64,
    pub coefficient_l: f64,

    pub min_applicable_temperature: f64,
    pub min_applicable_temperature_unit: String,
    pub max_applicable_temperature: f64,
    pub max_applicable_temperature_unit: String,

    pub is_show: i32,
    pub is_default: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
