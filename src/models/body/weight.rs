//! Weight logging API

use chrono::naive::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogWeightResponse {
    pub weight_log: Vec<WeightLog>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeightLog {
    pub bmi: f32,
    pub date: NaiveDate,
    pub log_id: usize,
    pub time: NaiveTime,
    pub weight: f32,
    pub source: String, // TODO: Device enum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_get_weight_logs() {
        let data = r#"
{
    "bmi":23.57,
    "date":"2015-03-05",
    "logId":1330991999000,
    "time":"23:59:59",
    "weight":73,
    "source": "API"
}
        "#;

        let _res: WeightLog = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn deserialize_log_weight() {
        let data = r#"
{
    "weightLog": [
    {
        "bmi": 23.57,
        "date": "2012-03-05",
        "logId": 1330991999000,
        "time": "23:59:59",
        "weight": 73,
        "source": "API"
    }
    ]
}
        "#;

        let _res: LogWeightResponse = serde_json::from_str(data).unwrap();
    }
}
