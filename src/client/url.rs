pub use std::collections::HashMap;

pub type Params<'a> = HashMap<&'a str, &'a str>;

pub const BASE: &str = "https://api.mozambiquehe.re";

#[non_exhaustive]
pub enum Endpoint {
    /// This allows you to get a player's statistics using his name and his platform.
    /// For PC players, you must use the Origin account name,
    /// even if they are playing on Steam (in that case, use the Origin account-
    /// name linked to that Steam account).
    /// Additional parameters are available.
    #[allow(unused)]
    Player,
    /// This API is currently unavailable to new users.  
    /// This API allows you to get the data for the top 500 players for each statistics
    Leaderboards,
    /// The map rotation API will return the current and next map for Battle Royale and Arenas,
    /// for both pubs and ranked modes.
    ///
    /// Control map rotation is also available. Be carefu
    MapRotation,
    /// This API will return the RP/AP needed to reach Apex Predator on
    /// PC, Playstation, Xbox and Switch.
    /// It will also return the number of Masters on each platform.
    Predator,
    /// Returns current in game shop data.
    /// but now, it seems not available
    #[deprecated(note = "No longer supported")]
    #[allow(unused)]
    Store,
    // /// This API will return the current items that can be crafted in replicators.
    // CraftingRotation,

    // /// Will return the latest news from the news feed in the given lang.
    // /// Lang defaults to en-US.
    // News,
    
    /// Shows the current server status, as shown on the apexlegendsstatus.com website
    ServerStatus,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Endpoint::Player => "bridge",
            Endpoint::Leaderboards => "leaderboard",
            Endpoint::MapRotation => "maprotation",
            Endpoint::Predator => "predator",
            // Endpoint::Store => "store",
            // Endpoint::CraftingRotation => "crafting",
            // Endpoint::News => "news",
            Endpoint::ServerStatus => "servers",
            _ => "",
        }
        .to_string()
    }
}

impl Endpoint {
    pub fn to_url(&self) -> String {
        let param = self.to_string();
        format!("{BASE}/{param}")
    }

    pub fn mix_with_params(&self, params: Params) -> String {
        let params = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{params}", self.to_url())
    }
}
