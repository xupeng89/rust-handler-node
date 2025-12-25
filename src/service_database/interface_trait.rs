use sea_orm::entity::*;
// 定义一个 Trait，让不同的 Model 都能返回 ID
pub trait HasId {
    fn get_id(&self) -> i32;
}

pub trait SyncableBinaryEntity: EntityTrait {
    fn col_i() -> String {
        "component_i_id".into()
    }
    fn col_j() -> String {
        "component_j_id".into()
    }
}
// 必须加上 pub
pub trait SyncableBinaryActiveModel: ActiveModelTrait {
    fn set_ids(&mut self, i_id: i32, j_id: i32);
    fn sync_set_from_json(&mut self, json: serde_json::Value) -> Result<(), sea_orm::DbErr>;
}
#[macro_export]
macro_rules! impl_binary_syncable {
    ($model:ty,$active_model:ty, $entity:ty) => {
        impl HasId for $model {
            fn get_id(&self) -> i32 {
                self.id
            } // 假设你的表主键都叫 id
        }
        impl SyncableBinaryEntity for $entity {}

        // 3. 为 ActiveModel 实现设置 ID 的逻辑
        impl $crate::service_database::interface_trait::SyncableBinaryActiveModel
            for $active_model
        {
            fn set_ids(&mut self, i_id: i32, j_id: i32) {
                // 这里的字段名必须与你生成的 ActiveModel 成员名一致
                self.component_i_id = sea_orm::ActiveValue::Set(i_id);
                self.component_j_id = sea_orm::ActiveValue::Set(j_id);
            }
            // 这个方法是公共的
            fn sync_set_from_json(
                &mut self,
                json: serde_json::Value,
            ) -> Result<(), sea_orm::DbErr> {
                let i_id = json["componentIId"].as_i64().unwrap_or(0) as i32;
                let j_id = json["componentJId"].as_i64().unwrap_or(0) as i32;

                self.set_ids(i_id, j_id); // 复用之前定义的 set_ids 方法

                if let Some(v) = json["componentI"].as_str() {
                    self.component_i = sea_orm::ActiveValue::Set(v.to_string());
                }
                if let Some(v) = json["componentJ"].as_str() {
                    self.component_j = sea_orm::ActiveValue::Set(v.to_string());
                }

                Ok(())
            }
        }
    };
}
#[macro_export]
macro_rules! sync_physical_calc_data {
    ($txn:expr, $incoming_ids:expr, $data:expr, $entity:path, $active_model:path, $item:ident, $am:ident, $assign_block:block) => {{
            // A. 删除不在传入列表中的数据
            <$entity as EntityTrait>::delete_many()
                .filter(<$entity as EntityTrait>::Column::Id.is_not_in($incoming_ids.clone()))
                .exec($txn)
                .await?;

            // B. 循环处理每一条数据
            for $item in $data {
                if let Some(id) = $item["id"].as_i64().map(|v| v as i32) {
                    let existing = <$entity as EntityTrait>::find_by_id(id).one($txn).await?;


                    let mut $am = <$active_model as Default>::default();
                    // 统一设置 ID
                    $am.set(<$entity as EntityTrait>::Column::Id, id.into());

                    // 执行外部传入的字段赋值逻辑
                    $assign_block

                    if existing.is_some() {
                        $am.update($txn).await?;
                    } else {
                        $am.insert($txn).await?;
                    }
                }
            }
        }};
}
