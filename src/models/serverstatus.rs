use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    #[serde(rename = "Origin_login")]
    pub origin_login: Detail,
    #[serde(rename = "EA_novafusion")]
    pub ea_novafusion: Detail,
    #[serde(rename = "EA_accounts")]
    pub ea_accounts: Detail,
    #[serde(rename = "ApexOauth_Crossplay")]
    pub apex_oauth_crossplay: Detail,
    pub self_core_test: SelfCoreTest,
    pub other_platforms: OtherPlatforms,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(rename = "EU-West")]
    pub eu_west: Local,
    #[serde(rename = "EU-East")]
    pub eu_east: Local,
    #[serde(rename = "US-West")]
    pub us_west: Local,
    #[serde(rename = "US-Central")]
    pub us_central: Local,
    #[serde(rename = "US-East")]
    pub us_east: Local,
    #[serde(rename = "SouthAmerica")]
    pub south_america: Local,
    #[serde(rename = "Asia")]
    pub asia: Local,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Local {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "HTTPCode")]
    pub httpcode: i64,
    #[serde(rename = "ResponseTime")]
    pub response_time: i64,
    #[serde(rename = "QueryTimestamp")]
    pub query_timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfCoreTest {
    #[serde(rename = "Status-website")]
    pub status_website: SelfCoreTestElement,
    #[serde(rename = "Stats-API")]
    pub stats_api: SelfCoreTestElement,
    #[serde(rename = "Overflow-#1")]
    pub overflow_1: SelfCoreTestElement,
    #[serde(rename = "Overflow-#2")]
    pub overflow_2: SelfCoreTestElement,
    #[serde(rename = "Origin-API")]
    pub origin_api: SelfCoreTestElement,
    #[serde(rename = "Playstation-API")]
    pub playstation_api: SelfCoreTestElement,
    #[serde(rename = "Xbox-API")]
    pub xbox_api: SelfCoreTestElement,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfCoreTestElement {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "HTTPCode")]
    pub httpcode: i64,
    #[serde(rename = "ResponseTime")]
    pub response_time: i64,
    #[serde(rename = "QueryTimestamp")]
    pub query_timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherPlatforms {
    #[serde(rename = "Playstation-Network")]
    pub playstation_network: CSNetwork,
    #[serde(rename = "Xbox-Live")]
    pub xbox_live: CSNetwork,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSNetwork {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "QueryTimestamp")]
    pub query_timestamp: i64,
}
