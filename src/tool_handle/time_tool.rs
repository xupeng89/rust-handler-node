use chrono::{NaiveDateTime, TimeZone, Utc};

pub fn naive_dt_utc_to_millis(naive_dt: NaiveDateTime) -> String {
    let dt_utc_plus_8 = Utc
        .from_local_datetime(&naive_dt)
        .single()
        .expect("NaiveDateTime is out of range for the fixed offset");

    // 3. 计算标准的 UTC 毫秒时间戳
    dt_utc_plus_8.timestamp_millis().to_string()
}

// 毫秒时间戳 → NaiveDateTime（UTC）
pub fn millis_to_naive_dt_utc(millis: String) -> NaiveDateTime {
    let utc_dt = Utc.timestamp_millis_opt(str_to_millis(&millis)).unwrap();
    // 3. 提取 NaiveDateTime
    utc_dt.naive_utc()
}

// 3. 字符串 → 毫秒 u64
fn str_to_millis(s: &str) -> i64 {
    s.parse().unwrap()
}
