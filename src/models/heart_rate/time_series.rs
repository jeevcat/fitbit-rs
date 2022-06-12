//! Get heart rate time series data.

use chrono::naive::NaiveDate;
use serde::Deserialize;

#[allow(dead_code)]
/// Possible period ranges for heart rate data.
pub enum Period {
    OneDay,
    SevenDays,
    ThirtyDays,
    OneWeek,
    OneMonth,
}

impl ToString for Period {
    fn to_string(&self) -> String {
        match self {
            Period::OneDay => "1d".to_string(),
            Period::SevenDays => "7d".to_string(),
            Period::ThirtyDays => "30d".to_string(),
            Period::OneWeek => "1w".to_string(),
            Period::OneMonth => "1m".to_string(),
        }
    }
}

/// Heart rate zones time series response.
#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "activities-heart")]
    pub series: Vec<DayEntry>,
}

/// A heart rate entry for a particular day.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DayEntry {
    pub date_time: NaiveDate,
    pub value: Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub custom_heart_rate_zones: Vec<super::HeartRateZone>,
    pub heart_rate_zones: Vec<super::HeartRateZone>,
    pub resting_heart_rate: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
{
    "activities-heart": [
        {
            "dateTime": "2015-08-04",
            "value": {
                "customHeartRateZones": [],
                "heartRateZones": [
                    {
                        "caloriesOut": 740.15264,
                        "max": 94,
                        "min": 30,
                        "minutes": 593,
                        "name": "Out of Range"
                    },
                    {
                        "caloriesOut": 249.66204,
                        "max": 132,
                        "min": 94,
                        "minutes": 46,
                        "name": "Fat Burn"
                    },
                    {
                        "caloriesOut": 0,
                        "max": 160,
                        "min": 132,
                        "minutes": 0,
                        "name": "Cardio"
                    },
                    {
                        "caloriesOut": 0,
                        "max": 220,
                        "min": 160,
                        "minutes": 0,
                        "name": "Peak"
                    }
                ],
                "restingHeartRate": 68
            }
        }
    ]
}
        "#;

        let _res: Response = serde_json::from_str(data).unwrap();
    }
}
