use crate::service_database::database_business::db_business_connection::get_business_db;
use napi_derive::napi;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Unchanged, ExprTrait, QueryFilter, Set, entity::prelude::*,
};
use serde::{Deserialize, Serialize};
/// 通用二元交互参数DTO
#[napi(
    object,
    namespace = "modelFluidPackageBinary",
    js_name = "BinaryParameterDto"
)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinaryParameterDto {
    pub id: String,
    pub fluid_package_id: String,
    pub compound_i: String,
    pub compound_j: String,
    // 基础参数（Option 兼容不同表）
    pub aij: Option<String>,
    pub aji: Option<String>,
    pub bij: Option<String>,
    pub bji: Option<String>,
    pub cij: Option<String>,
    pub cji: Option<String>,
    pub dij: Option<String>,
    pub dji: Option<String>,
    pub eij: Option<String>,
    pub eji: Option<String>,
    pub fij: Option<String>,
    pub fji: Option<String>,
    // EOS 参数
    pub kaij: Option<String>,
    pub kbij: Option<String>,
    pub kcij: Option<String>,
    // PSRK 参数
    pub tij: Option<String>,
    pub tji: Option<String>,
    pub vij: Option<String>,
    pub vji: Option<String>,
    // 共有但部分表没有的物理约束字段 (报错的核心点)
    pub min_t: Option<String>, // 必须加上这个
    pub max_t: Option<String>, // 必须加上这个

    pub is_default: Option<i32>,
    pub is_default_id: Option<String>,
}

macro_rules! fill_binary_fields {
    // 类别 1: 完整版活动系数 (如 NRTL-RK, 部分 NRTL)
    (activity_f, $am:expr, $dto:expr) => {
        // fill_all_activity!($am, $dto);
        fill_common_activity!($am, $dto); // 填充 A, B, C
        if let Some(v) = &$dto.dij {
            $am.dij = Set(v.clone());
        }
        if let Some(v) = &$dto.eij {
            $am.eij = Set(v.clone());
        }
        if let Some(v) = &$dto.eji {
            $am.eji = Set(v.clone());
        }
        if let Some(v) = &$dto.fij {
            $am.fij = Set(v.clone());
        } // 有 F
        if let Some(v) = &$dto.fji {
            $am.fji = Set(v.clone());
        }
    };

    // 类别 2: Wilson/UNIQUAC 专用 (有 A-E 和 T，无 F)
    (activity_e, $am:expr, $dto:expr) => {
        // fill_all_activity!($am, $dto);
        fill_common_activity!($am, $dto); // 填充 A, B, C
        if let Some(v) = &$dto.cji {
            $am.cji = Set(v.clone());
        } // Wilson 有 CJI
        if let Some(v) = &$dto.dij {
            $am.dij = Set(v.clone());
        }
        if let Some(v) = &$dto.dji {
            $am.dji = Set(v.clone());
        }
        if let Some(v) = &$dto.eij {
            $am.eij = Set(v.clone());
        }
        if let Some(v) = &$dto.eji {
            $am.eji = Set(v.clone());
        }
    };

    // 类别 3: 状态方程类 (PR, RK, SRK)
    (eos, $am:expr, $dto:expr) => {
        // fill_all_activity!($am, $dto);
        if let Some(v) = &$dto.kaij {
            $am.kaij = Set(v.clone());
        }
        if let Some(v) = &$dto.kbij {
            $am.kbij = Set(v.clone());
        }
        if let Some(v) = &$dto.kcij {
            $am.kcij = Set(v.clone());
        }
        // if let Some(v) = &$dto.min_t {
        //     $am.min_t = Set(v.clone());
        // }
        // if let Some(v) = &$dto.max_t {
        //     $am.max_t = Set(v.clone());
        // }
    };

    // 类别 4: PSRK 专用
    (psrk, $am:expr, $dto:expr) => {
        // fill_all_activity!($am, $dto);
        if let Some(v) = &$dto.tij {
            $am.tij = Set(v.clone());
        }
        if let Some(v) = &$dto.tji {
            $am.tji = Set(v.clone());
        }
        if let Some(v) = &$dto.vij {
            $am.vij = Set(v.clone());
        }
        if let Some(v) = &$dto.vji {
            $am.vji = Set(v.clone());
        }
    };
}

// 提取公共的 A, B, C 赋值逻辑
macro_rules! fill_common_activity {
    ($am:expr, $dto:expr) => {
        if let Some(v) = &$dto.aij {
            $am.aij = Set(v.clone());
        }
        if let Some(v) = &$dto.aji {
            $am.aji = Set(v.clone());
        }
        if let Some(v) = &$dto.bij {
            $am.bij = Set(v.clone());
        }
        if let Some(v) = &$dto.bji {
            $am.bji = Set(v.clone());
        }
        if let Some(v) = &$dto.cij {
            $am.cij = Set(v.clone());
        }

        // 增加 Wilson 特有的 T 范围
        if let Some(v) = &$dto.min_t {
            $am.min_t = Set(v.clone());
        }
        if let Some(v) = &$dto.max_t {
            $am.max_t = Set(v.clone());
        }
    };
}

macro_rules! impl_binary_service {
    ($service_name:ident, $entity:ident, $type:ident) => {
        pub struct $service_name;

        impl $service_name {
            /// 核心：将 DTO 转换为 ActiveModel (支持插入和更新)
            fn dto_to_active_model(
                item: BinaryParameterDto,
                is_update: bool,
            ) -> $entity::ActiveModel {
                // 1. 直接通过 from_json 或者手动字段构造
                // 由于 ActiveModel 没 Default，我们直接 new
                let mut am = $entity::ActiveModel {
                    id: if is_update {
                        Unchanged(item.id.clone())
                    } else {
                        Set(item.id.clone())
                    },
                    fluid_package_id: Set(item.fluid_package_id.clone()),
                    compound_i: Set(item.compound_i.clone()),
                    compound_j: Set(item.compound_j.clone()),

                    ..Default::default()
                };

                // 2. 填充其他业务字段
                if let Some(v) = item.is_default {
                    am.is_default = Set(v);
                }
                if let Some(v) = item.is_default_id {
                    am.is_default_id = Set(v.clone());
                }

                // 3. 调用字段填充宏
                fill_binary_fields!($type, &mut am, item);

                am
            }

            /// 批量保存 (Save: Insert or Update)
            pub async fn batch_save(datas: Vec<BinaryParameterDto>) -> Result<(), DbErr> {
                let db = get_business_db().await?;
                for item in datas {
                    let id = item.id.clone();
                    let exists = $entity::Entity::find_by_id(id).one(db).await?.is_some();

                    let am = Self::dto_to_active_model(item, exists);
                    if exists {
                        am.update(db).await?;
                    } else {
                        am.insert(db).await?;
                    }
                }
                Ok(())
            }

            /// 根据 FluidPackageId 查询所有参数
            pub async fn find_by_package_id(
                package_id: String,
            ) -> Result<Vec<$entity::Model>, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::find()
                    .filter($entity::Column::FluidPackageId.eq(package_id))
                    .all(db)
                    .await
            }

            /// 根据 FluidPackageId 和default 查询所有参数
            pub async fn find_by_package_id_and_default(
                package_id: String,
                is_default: u32,
            ) -> Result<Vec<$entity::Model>, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::find()
                    .filter($entity::Column::FluidPackageId.eq(package_id))
                    .filter($entity::Column::IsDefault.eq(is_default))
                    .all(db)
                    .await
            }

            /// 根据 FluidPackageId 和default 查询所有参数
            pub async fn get_all_binary_parameter_by_package_ids_default(
                package_ids: Vec<String>,
                is_default: u32,
            ) -> Result<Vec<$entity::Model>, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::find()
                    .filter($entity::Column::FluidPackageId.is_in(package_ids))
                    .filter($entity::Column::IsDefault.eq(is_default))
                    .all(db)
                    .await
            }
            /// 根据 FluidPackageId 和default 查询所有参数
            pub async fn get_all_binary_parameter_by_package_ids(
                package_ids: Vec<String>,
            ) -> Result<Vec<$entity::Model>, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::find()
                    .filter($entity::Column::FluidPackageId.is_in(package_ids))
                    .all(db)
                    .await
            }

            pub async fn find_by_ids(ids: Vec<String>) -> Result<Vec<$entity::Model>, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::find()
                    .filter($entity::Column::Id.is_in(ids))
                    .all(db)
                    .await
            }

            pub async fn delete_by_package_id_and_cas_nos(
                package_id: String,
                cas_nos: Vec<String>,
            ) -> Result<bool, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::delete_many()
                    .filter($entity::Column::FluidPackageId.eq(package_id))
                    .filter(
                        $entity::Column::CompoundI
                            .is_in(cas_nos.clone())
                            .or($entity::Column::CompoundJ.is_in(cas_nos)),
                    )
                    .exec(db)
                    .await?;
                Ok(true)
            }

            /// 根据 FluidPackageId 删除所有相关参数
            pub async fn delete_by_package_id(package_id: String) -> Result<bool, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::delete_many()
                    .filter($entity::Column::FluidPackageId.eq(package_id))
                    .exec(db)
                    .await?;
                Ok(true)
            }
            pub async fn delete_by_package_ids(package_ids: Vec<String>) -> Result<bool, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::delete_many()
                    .filter($entity::Column::FluidPackageId.is_in(package_ids))
                    .exec(db)
                    .await?;
                Ok(true)
            }

            pub async fn delete_by_ids_and_func_code_no_default(
                ids: Vec<String>,
            ) -> Result<bool, DbErr> {
                let db = get_business_db().await?;
                $entity::Entity::delete_many()
                    .filter($entity::Column::Id.is_in(ids))
                    .exec(db)
                    .await?;
                Ok(true)
            }
            pub async fn delete_by_ids_and_func_code_has_default(
                ids: Vec<String>,
            ) -> Result<bool, DbErr> {
                let db = get_business_db().await?;
                let entity_list = $entity::Entity::find()
                    .filter($entity::Column::Id.is_in(&ids))
                    .all(db)
                    .await?;
                // 3. 过滤出非空的 is_default_id（避免删除空 ID）
                let mut all_ids = ids; // 直接重用传入的 ids 向量

                for entity in entity_list {
                    all_ids.push(entity.is_default_id);
                }
                all_ids.sort();
                // 去重（避免重复删除同一 ID，提升性能）
                all_ids.dedup();

                // 5. 边界处理：无需要删除的 ID 时直接返回 false
                if all_ids.is_empty() {
                    return Ok(false);
                }

                // 6. 批量删除合并后的所有 ID
                let delete_result = $entity::Entity::delete_many()
                    .filter($entity::Column::Id.is_in(all_ids))
                    .exec(db) // 使用引用
                    .await?;

                // 7. 验证删除结果（可选：确认是否有记录被删除）
                let is_success = delete_result.rows_affected > 0;

                Ok(is_success)
            }
        }
    };
}

// 导入所有生成的实体
use crate::service_database::database_business::entity::fluid_package::{
    model_fluid_package_binary_nrtl_entity as nrtl,
    model_fluid_package_binary_nrtl_rk_entity as nrtl_rk,
    model_fluid_package_binary_pr_entity as pr, model_fluid_package_binary_psrk_entity as psrk,
    model_fluid_package_binary_rk_entity as rk, model_fluid_package_binary_srk_entity as srk,
    model_fluid_package_binary_uniquac_entity as uniquac,
    model_fluid_package_binary_wilsion_entity as wilson,
};

// 1. Wilson 和 UNIQUAC 使用 activity_e (有 E 无 F)
impl_binary_service!(WilsonService, wilson, activity_e);
impl_binary_service!(UniquacService, uniquac, activity_e);

// 2. NRTL 和 NRTL-RK 使用 activity_f (根据你之前的报错，它们应该有 F)
impl_binary_service!(NrtlRkService, nrtl_rk, activity_f);
impl_binary_service!(NrtlService, nrtl, activity_f);

// 3. 状态方程类
impl_binary_service!(PrService, pr, eos);
impl_binary_service!(RkService, rk, eos);
impl_binary_service!(SrkService, srk, eos);

// 4. PSRK
impl_binary_service!(PsrkService, psrk, psrk);
