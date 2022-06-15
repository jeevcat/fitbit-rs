//! Body fat logs

use chrono::naive::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FatLog {
    pub date: NaiveDate,
    pub fat: f32,
    pub log_id: u64,
    pub time: NaiveTime,
    pub source: String, // TODO: Device enum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
{
    "date":"2012-03-05",
    "fat":13.5,
    "logId":1330991999000,
    "time":"21:20:59",
    "source":"Aria"
}
        "#;

        let _res: FatLog = serde_json::from_str(data).unwrap();
    }
}
