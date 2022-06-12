//! Get activity time series data.

use chrono::naive::NaiveDate;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Resource {
    Calories,
    CaloriesBMR,
    Steps,
    Distance,
    Floors,
    Elevation,
    Sedentary,
    LightlyActive,
    FairlyActive,
    VeryActive,
    ActivityCalories,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub date_time: NaiveDate,
    pub value: String,
}

macro_rules! endpoint {
    ($mod:ident, $rename:expr) => {
        pub mod $mod {
            use serde::Deserialize;

            #[derive(Deserialize, Debug)]
            pub struct Response {
                #[serde(rename = $rename)]
                pub series: Vec<super::Entry>,
            }
        }
    };
}

endpoint!(calories, "activities-calories");
endpoint!(calories_bmr, "activities-caloriesBMR");
endpoint!(steps, "activities-steps");
endpoint!(distance, "activities-distance");
endpoint!(floors, "activities-floors");
endpoint!(elevation, "activities-elevation");
endpoint!(minutes_sedentary, "activities-minutesSedentary");
endpoint!(minutes_lightly_active, "activities-minutesLightlyActive");
endpoint!(minutes_fairly_active, "activities-minutesFairlyActive");
endpoint!(minutes_very_active, "activities-minutesVeryActive");
endpoint!(activity_calories, "activities-activityCalories");
