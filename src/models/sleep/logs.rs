//! Get sleep logs for a user.
//!
//! [More information?](https//dev.fitbit.com/build/reference/web-api/sleep/#
//! get-sleep-logs)

use serde::Deserialize;

/// Get sleep logs response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub sleep: Vec<SleepEntry>,
    pub summary: Option<Summary>,
}

/// A sleep entry for a particular day.
///
/// There can be multiple entries per day.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SleepEntry {}

/// A sleep summary.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub total_minutes_asleep: usize,
    pub total_sleep_records: usize,
    pub total_time_in_bed: usize,
}
