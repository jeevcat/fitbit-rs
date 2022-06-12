//! User profile APIs

use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod update;

/// A user profile response from a GET or POST request.
/// Both the get and update APIs send this back as a response.
#[derive(Deserialize, Debug)]
pub struct Response {
    /// All information about a user profile.
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub about_me: Option<String>,
    pub age: usize,
    pub ambassador: bool,
    pub auto_stride_enabled: Option<bool>,
    pub avatar: Url,
    pub avatar150: Url,
    pub avatar640: Url,
    pub average_daily_steps: usize,
    pub clock_time_display_format: ClockTimeDisplayFormat,
    pub corporate: bool,
    pub corporate_admin: bool,
    pub country: Option<String>,
    pub date_of_birth: NaiveDate,
    pub display_name: String,
    pub display_name_setting: String,
    pub distance_unit: DistanceUnit,
    pub email: Option<String>,
    pub encoded_id: String,
    pub family_guidance_enabled: Option<bool>,
    pub first_name: String,
    pub foods_locale: String,
    pub full_name: String,
    pub gender: Gender,
    pub glucose_unit: GlucoseUnit,
    pub height: f32,
    pub height_unit: HeightUnit,
    pub is_child: bool,
    pub last_name: String,
    pub locale: String,
    pub member_since: NaiveDate,
    pub mfa_enabled: bool,
    #[serde(rename = "offsetFromUTCMillis")]
    pub offset_from_utc: i64,
    pub start_day_of_the_week: Option<StartDayOfTheWeek>,
    pub stride_length_running: f32,
    pub stride_length_running_type: String,
    pub stride_length_walking: f32,
    pub stride_length_walking_type: String,
    pub swim_unit: SwimUnit,
    pub timezone: String,
    pub water_unit: WaterUnit,
    pub water_unit_name: String,
    pub weight: f32,
    pub weight_unit: WeightUnit,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClockTimeDisplayFormat {
    #[serde(rename = "12hour")]
    Hours12,
    #[serde(rename = "24hour")]
    Hours24,
}

#[derive(Deserialize, Debug)]
pub enum DistanceUnit {
    #[serde(rename = "en_US")]
    FeetInches,
    #[serde(rename = "METRIC")]
    Centimeters,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Gender {
    Male,
    Female,
    NA,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GlucoseUnit {
    #[serde(rename = "en_US")]
    Imperial,
    Metric,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum HeightUnit {
    #[serde(rename = "en_US")]
    Imperial,
    Metric,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StartDayOfTheWeek {
    Sunday,
    Monday,
}

#[derive(Deserialize, Debug)]
pub enum SwimUnit {
    #[serde(rename = "en_US")]
    Yards,
    #[serde(rename = "METRIC")]
    Meters,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum WaterUnit {
    #[serde(rename = "en_US")]
    FluidOuncesOrCups,
    #[serde(rename = "METRIC")]
    Milliliters,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum WeightUnit {
    #[serde(rename = "en_US")]
    Pounds,
    #[serde(rename = "en_GB")]
    Stone,
    #[serde(rename = "METRIC")]
    Kilograms,
}
