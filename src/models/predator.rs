use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "RP")]
    pub rp: Rp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rp {
    #[serde(rename = "PC")]
    pub pc: Agent,
    #[serde(rename = "PS4")]
    pub ps4: Agent,
    #[serde(rename = "X1")]
    pub x1: Agent,
    #[serde(rename = "SWITCH")]
    pub switch: Agent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub found_rank: i64,
    pub val: i64,
    pub uid: String,
    pub update_timestamp: i64,
    pub total_masters_and_preds: i64,
}
