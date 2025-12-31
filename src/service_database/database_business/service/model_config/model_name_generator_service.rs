use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_name_generator_entity::{ActiveModel as NameGenActiveModel,Entity as NameGenEntity,Model as NameGenModel,Column as NameGenColumn};

use napi_derive::napi;
use sea_orm::{QueryFilter, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[napi(object, namespace = "nameGenerator", js_name = "NameGenDTO")]
#[derive(Serialize, Deserialize)]
pub struct NameGenDTO {
    pub id: i32,
    pub code: String,
    pub number_segment: String,
    pub model_id: String,
}

impl From<NameGenModel> for NameGenDTO {
    fn from(c: NameGenModel) -> Self {
        NameGenDTO {
            id: c.id,
            model_id: c.model_id,
            code: c.code,
            number_segment: c.number_segment,
        }
    }
}

impl From<NameGenDTO> for NameGenActiveModel {
    fn from(data: NameGenDTO) -> Self {
        Self {
            id: Set(data.id),
            model_id: Set(data.model_id),
            code: Set(data.code),
            number_segment: Set(data.number_segment),
        }
    }
}

// --- 内部算法工具函数 ---

/// 对应 TS 的 insertSingleNumber: 插入一个数字并合并号段
pub fn insert_single_number(num: i32, ranges_str: &str) -> String {
    if ranges_str.is_empty() {
        return format!("{}-{}", num, num);
    }

    let mut ranges: Vec<(i32, i32)> = ranges_str
        .split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() == 2 {
                Some((parts[0].parse().unwrap_or(0), parts[1].parse().unwrap_or(0)))
            } else {
                None
            }
        })
        .collect();

    ranges.push((num, num));
    ranges.sort_by_key(|k| k.0);

    let mut result: Vec<(i32, i32)> = Vec::new();
    for curr in ranges {
        if let Some(last) = result.last_mut() {
            if curr.0 <= last.1 + 1 {
                last.1 = last.1.max(curr.1);
            } else {
                result.push(curr);
            }
        } else {
            result.push(curr);
        }
    }
    result
        .iter()
        .map(|(s, e)| format!("{}-{}", s, e))
        .collect::<Vec<_>>()
        .join(",")
}

/// 对应 TS 的 removeNumberFromRanges: 从号段中移除一个数字
pub fn remove_number_from_ranges(num: i32, ranges_str: &str) -> String {
    let mut new_ranges = Vec::new();
    for range_str in ranges_str.split(',') {
        let parts: Vec<i32> = range_str
            .split('-')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        if parts.len() < 2 {
            continue;
        }
        let (start, end) = (parts[0], parts[1]);

        if num < start || num > end {
            new_ranges.push(range_str.to_string());
        } else if num == start && num == end {
            continue;
        } else if num == 1 && num == start {
            new_ranges.push(format!("0-{}", end));
        } else if num == start {
            new_ranges.push(format!("{}-{}", start + 1, end));
        } else if num == end {
            new_ranges.push(format!("{}-{}", start, end - 1));
        } else {
            new_ranges.push(format!("{}-{}", start, num - 1));
            new_ranges.push(format!("{}-{}", num + 1, end));
        }
    }
    new_ranges.join(",")
}

fn get_number_from_name(name: &str) -> i32 {
    let re = regex::Regex::new(r"\d+").unwrap();
    re.find(name)
        .map(|m| m.as_str().parse().unwrap_or(-1))
        .unwrap_or(-1)
}

// --- 数据库 Service ---

use std::collections::HashMap;
use std::sync::OnceLock;

// 静态前缀映射表
static UNIT_CODE_PREFIX: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn get_unit_prefix_map() -> &'static HashMap<&'static str, &'static str> {
    UNIT_CODE_PREFIX.get_or_init(|| {
        let mut m = HashMap::new();
        // 核心单元与物流
        m.insert("seperator", "V");
        m.insert("simpleDistTower", "T");
        m.insert("heatX", "E");
        m.insert("mixerTee", "MT");
        m.insert("pump", "P");
        m.insert("valve", "VLV");
        m.insert("material", "MS");
        m.insert("energy", "Q");
        m.insert("airCooler", "A");
        m.insert("furnace", "F");
        m.insert("pipe", "PIPE");

        // 反应器与压缩机
        m.insert("conversionReactor", "R");
        m.insert("centrifugalCompressor", "C");

        // 逻辑计算与信号
        m.insert("signal", "SI");
        m.insert("byPass", "BP");
        m.insert("vote", "NSM");
        m.insert("rs", "RS");
        m.insert("not", "INV");
        m.insert("andOr", "LO");
        m.insert("delay", "DLY");
        m.insert("pulse", "PLS");
        m.insert("positionInformation", "PI");

        // 传感器 (Sensors)
        m.insert("tSensor", "TG");
        m.insert("compSensor", "CG");
        m.insert("fSensor", "FG");
        m.insert("nvfSensor", "NVFG");
        m.insert("vfSensor", "VFG");
        m.insert("pSensor", "PG");
        m.insert("lSensor", "LG");
        m.insert("dSensor", "DG");

        // 算法、脚本与包
        m.insert("aiuo", "AI");
        m.insert("reactionPackage", "Rxn");
        m.insert("cutter", "CUT");
        m.insert("scriptLogic", "LOGIC");
        m.insert("scriptUO", "UO");

        m
    })
}

/// 格式化数字为 001 这种形式 (对应 TS 的 formatNumber)
fn format_number(num: i32) -> String {
    format!("{:03}", num)
}

/// 处理双号段逻辑 (对应 TS 的 getDoubleNumberSegment)
fn get_double_number_segment(list: Vec<String>) -> (String, String) {
    // 逻辑与 TS 保持一致：取前两个号段进行判断
    let first = list.get(0).cloned().unwrap_or_else(|| "0-0".to_string());
    let second = list.get(1).cloned().unwrap_or_else(|| "0-0".to_string());
    let rest = if list.len() > 2 {
        list[2..].to_vec()
    } else {
        vec![]
    };

    let first_parts: Vec<i32> = first.split('-').map(|s| s.parse().unwrap_or(0)).collect();
    let second_parts: Vec<i32> = second.split('-').map(|s| s.parse().unwrap_or(0)).collect();

    let f_start = *first_parts.get(0).unwrap_or(&0);
    let f_end = *first_parts.get(1).unwrap_or(&0);
    let s_start = *second_parts.get(0).unwrap_or(&0);
    let s_end = *second_parts.get(1).unwrap_or(&0);

    let mut number_segment_res = Vec::new();
    let number_add;
    let gap = s_start - f_end;

    if gap > 2 {
        number_add = f_end + 1;
        number_segment_res.push(format!("{}-{}", f_start, number_add));
        number_segment_res.push(second);
        number_segment_res.extend(rest);
    } else if gap == 2 {
        number_add = f_end + 1;
        number_segment_res.push(format!("{}-{}", f_start, s_end));
        number_segment_res.extend(rest);
    } else {
        number_add = 0;
        // 如果 gap 不满足条件，保持原样或按业务逻辑处理
        number_segment_res.push(first);
        number_segment_res.push(second);
        number_segment_res.extend(rest);
    }

    (number_segment_res.join(","), format_number(number_add))
}

/// 处理单号段逻辑 (对应 TS 的 getEndByNumberSegment)
fn get_end_by_number_segment(number_segment: &str) -> (String, String) {
    if number_segment == "0-0" || number_segment.is_empty() {
        return ("1-1".to_string(), "001".to_string());
    }

    let parts: Vec<i32> = number_segment
        .split('-')
        .map(|s| s.parse().unwrap_or(1))
        .collect();
    let start = *parts.get(0).unwrap_or(&1);
    let last_end = *parts.last().unwrap_or(&0);

    if start == 0 {
        (format!("1-{}", last_end), format_number(1))
    } else if start == 1 {
        let new_end = last_end + 1;
        (format!("1-{}", new_end), format_number(new_end))
    } else {
        let name_val = start - 1;
        (
            format!("{}-{}", name_val, last_end),
            format_number(name_val),
        )
    }
}

/// 核心函数：根据号段获取名称 (对应 TS 的 getGraphNameByNumberSegment)
#[napi(object)]
pub struct NameResult {
    pub number_segment: String,
    pub name: String,
}

pub fn get_graph_name_logic(code: String, number_segment: String) -> NameResult {
    let list: Vec<String> = if number_segment.is_empty() || number_segment == "0-0" {
        vec!["0-0".to_string()]
    } else {
        number_segment.split(',').map(|s| s.to_string()).collect()
    };

    let (final_segment, raw_name) = if list.len() > 1 {
        get_double_number_segment(list)
    } else {
        get_end_by_number_segment(&number_segment)
    };

    // 获取前缀
    let prefixes = get_unit_prefix_map();
    let prefix = prefixes.get(code.as_str()).unwrap_or(&"");

    // 注意：这里你可以根据需要加入你 TS 中 modelUnitCodeStartWith 的复杂 Replace 逻辑
    // 暂时按你 modelUnitCodePrefix[code] + result.name 的逻辑实现
    NameResult {
        number_segment: final_segment,
        name: format!("{}{}", prefix, raw_name),
    }
}

pub async fn get_used_name_detail(code: String, model_id: String) -> Result<NameGenDTO, DbErr> {
    let db = get_business_db().await?;
    let model = NameGenEntity::find()
        .filter(NameGenColumn::Code.eq(code))
        .filter(NameGenColumn::ModelId.eq(model_id))
        .one(db)
        .await?;

    let result = model.map(NameGenDTO::from).unwrap();
    Ok(result)
}

pub async fn update_number_segment_delete(
    code: String,
    name_delete: String,
    model_id: String,
) -> Result<(), DbErr> {
    let db = get_business_db().await?;
    let service_result = get_used_name_detail(code, model_id).await?;
    let num = get_number_from_name(&name_delete);
    let new_segment = remove_number_from_ranges(num, &service_result.number_segment);
    let mut active: NameGenActiveModel = service_result.into();
    active.number_segment = Set(new_segment);
    active.update(db).await?;

    Ok(())
}

pub async fn update_number_segment_name_number(
    code: String,
    name_number: String,
    model_id: String,
) -> Result<(), DbErr> {
    let db = get_business_db().await?;
    let service_result = get_used_name_detail(code, model_id).await?;
    let num = get_number_from_name(&name_number);
    let new_segment = insert_single_number(num, &service_result.number_segment);
    let mut active: NameGenActiveModel = service_result.into();
    active.number_segment = Set(new_segment);
    active.update(db).await?;

    Ok(())
}

pub async fn insert_or_update_name_business(
    code: String,
    segment: String,
    model_id: String,
) -> Result<(), DbErr> {
    let db = get_business_db().await?;

    // 1. 尝试获取现有记录
    // 注意：这里去掉了 ?，因为我们需要手动处理 Err 逻辑
    let existing_result = get_used_name_detail(code.clone(), model_id.clone()).await;

    match existing_result {
        // 情况 A：查询成功，说明记录存在，执行更新
        Ok(res) => {
            // 注意：如果 res 是 DTO，确保它实现了 into ActiveModel 且带有 ID
            let mut active: NameGenActiveModel = res.into();
            active.number_segment = Set(segment);
            active.update(db).await?;
        }
        // 情况 B：查询报错
        Err(e) => {
            // 如果错误是因为“找不到记录”，则执行插入逻辑
            if let DbErr::RecordNotFound(_) = e {
                let active = NameGenActiveModel {
                    code: Set(code),
                    number_segment: Set(segment),
                    model_id: Set(model_id),
                    ..Default::default()
                };
                active.insert(db).await?;
            } else {
                // 如果是其他数据库错误（如连接断开），则继续向上抛出
                return Err(e);
            }
        }
    }

    Ok(())
}
