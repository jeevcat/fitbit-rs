//! # Sleep Goals APIs

use chrono::DateTime;
use serde::Deserialize;

pub mod get;
pub mod update;

/// Information about the current sleep goal.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub min_duration: usize,
    // TODO: Not sure what the correct timezone is here...
    pub updated_on: DateTime<chrono::Utc>,
}
