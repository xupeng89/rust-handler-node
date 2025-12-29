use chrono::{FixedOffset, LocalResult, NaiveDateTime, TimeZone, Utc};

// 时间戳使用功能

pub fn naive_dt_utc_to_millis(naive_dt: NaiveDateTime) -> String {
    let dt_utc_plus_8 = Utc
        .from_local_datetime(&naive_dt)
        .single()
        .expect("NaiveDateTime is out of range for the fixed offset");

    // 3. 计算标准的 UTC 毫秒时间戳
    dt_utc_plus_8.timestamp_millis().to_string()
}

// 毫秒时间戳 → NaiveDateTime（UTC）
// pub fn millis_to_naive_dt_utc(millis: String) -> NaiveDateTime {
//     let utc_dt = Utc.timestamp_millis_opt(str_to_millis(&millis)).unwrap();
//     // 3. 提取 NaiveDateTime
//     utc_dt.naive_utc()
// }

// // 3. 字符串 → 毫秒 u64
// fn str_to_millis(s: &str) -> i64 {
//     s.parse().unwrap()
// }

pub fn integer_to_string(timestamp_ms: i64) -> String {
    // UTC 时间，非法时间戳 fallback 到 1970-01-01 00:00:00
    let dt_utc = match Utc.timestamp_millis_opt(timestamp_ms) {
        LocalResult::Single(dt) => dt,
        _ => Utc.timestamp_millis_opt(0).unwrap(),
    };

    // 东八区，安全 fallback 到 UTC+8
    let dt_east8 = FixedOffset::east_opt(8 * 3600)
        .map(|offset| dt_utc.with_timezone(&offset))
        .unwrap();

    // 格式化输出，安全返回
    dt_east8.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let ts = 1766896693918; // 毫秒时间戳
        let formatted = integer_to_string(ts);
        println!("东八区时间: {}", formatted);
    }
}
