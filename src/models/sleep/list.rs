//! Get sleep list for a user.

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pagination: Pagination,
    sleep: Vec<Sleep>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    before_date: Option<String>,
    after_date: Option<String>,
    limit: u64,
    next: String,
    offset: u64,
    previous: String,
    sort: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sleep {
    date_of_sleep: String,
    #[serde(rename = "duration")]
    duration_millis: u64,
    efficiency: u64,
    end_time: String,
    info_code: u64,
    is_main_sleep: bool,
    levels: Levels,
    log_id: u64,
    minutes_after_wakeup: u64,
    minutes_asleep: u64,
    minutes_awake: u64,
    minutes_to_fall_asleep: u64,
    start_time: String,
    time_in_bed: u64,
    #[serde(rename = "type")]
    type_: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Levels {
    summary: LevelSummary,
    data: Vec<SleepDataPoint>,
    short_data: Vec<SleepDataPoint>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LevelSummary {
    deep: Summary,
    light: Summary,
    rem: Summary,
    wake: Summary,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    count: u64,
    minutes: u64,
    thirty_day_avg_minutes: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SleepDataPoint {
    date_time: String,
    level: SleepLevel,
    seconds: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SleepLevel {
    Wake,
    Light,
    Rem,
    Deep,
}
