use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllMap {
    pub battle_royale: Map,
    pub ranked: Map,
    #[serde(rename = "ltm")]
    pub other: Map,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub current: Current,
    pub next: Next,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub start: i64,
    pub end: i64,
    #[serde(rename = "readableDate_start")]
    pub readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    pub readable_date_end: String,
    pub map: String,
    pub code: String,
    #[serde(rename = "DurationInSecs")]
    pub duration_in_secs: i64,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: i64,
    pub asset: String,
    pub remaining_secs: i64,
    pub remaining_mins: i64,
    pub remaining_timer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    pub start: i64,
    pub end: i64,
    #[serde(rename = "readableDate_start")]
    pub readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    pub readable_date_end: String,
    pub map: String,
    pub code: String,
    #[serde(rename = "DurationInSecs")]
    pub duration_in_secs: i64,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: i64,
}
