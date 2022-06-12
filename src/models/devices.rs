//! # Get Devices
//!
//! This [endpoint](https://dev.fitbit.com/build/reference/web-api/devices/#get-devices)
//! returns information about a user's paired devices.
//!
//! This API provides the following (interesting) information per device:
//! - Device product name
//! - Battery level indications:
//!     - A general `High`, `Medium`, `Low` or `Empty` indication
//!     - A battery percentage
//! - Last sync time

use chrono::naive::NaiveDateTime;
use serde::Deserialize;

/// Expected response container.
#[derive(Deserialize, Debug)]
pub struct Response(pub Vec<DeviceInfo>);

/// Information about a particular device.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    /// Rough battery level indication.
    pub battery: BatteryLevel,
    /// Battery percentage from last sync.
    #[serde(rename = "batteryLevel")]
    pub battery_percentage: usize,
    pub device_version: Device,
    /// Used by other endpoints.
    pub id: String,
    /// Last time the device sent data to Fitbit.
    pub last_sync_time: NaiveDateTime,
    /// Product name.
    #[serde(rename = "type")]
    pub ty: DeviceType,
}

/// Rough battery level indication.
#[derive(Deserialize, Debug, PartialEq, PartialOrd)]
pub enum BatteryLevel {
    Empty,
    Low,
    Medium,
    High,
}

/// Used to separate different product categories.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    /// Something you wear on your wrist.
    Tracker,
    /// A smart scale.
    Scale,
}

/// A Fitbit product.
#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub enum Device {
    Aria,
    #[serde(rename = "Charge 2")]
    Charge2,
    #[serde(rename = "Charge 3")]
    Charge3,
    #[serde(rename = "Charge 4")]
    Charge4,
    #[serde(rename = "Charge 5")]
    Charge5,
    #[serde(rename = "Charge HR")]
    ChargeHr,
    Ionic,
    MobileTrack,
    Surge,
    Versa,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
[
{
    "battery": "High",
    "batteryLevel": 100,
    "deviceVersion": "Charge HR",
    "id": "27072629",
    "lastSyncTime": "2015-07-27T17:01:39.313",
    "type": "TRACKER"
},
{
    "battery": "Empty",
    "batteryLevel": 100,
    "deviceVersion": "MobileTrack",
    "id": "29559794",
    "lastSyncTime": "2015-07-19T16:57:59.000",
    "type": "TRACKER"
},
{
    "battery": "High",
    "batteryLevel": 100,
    "deviceVersion": "Aria",
    "id": "Y1PFEJZGGX8QFYTV",
    "lastSyncTime": "2015-07-27T07:14:34.000",
    "type": "SCALE"
}
]
        "#;

        let res: Response = serde_json::from_str(data).unwrap();
        println!("{:#?}", res);
    }
}
