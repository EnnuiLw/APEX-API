use super::url::Endpoint;
use crate::{models, types::*};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait BaseClient {
    async fn inner_connect<T>(&self, endpoint: &str) -> crate::ClientResult<T>
    where
        T: DeserializeOwned;

    async fn player(&self) -> crate::ClientResult<()> {
        Ok(())
    }

    #[deprecated(note = "im not in whitelist yet fk.")]
    async fn leaderboards(&self, platform: Platform, legend: Character) -> crate::ClientResult<()> {
        let platform = platform.to_string();
        let legend = legend.to_string();
        let params = HashMap::from([("platform", &*platform), ("legend", &*legend)]);
        let _endpoint = Endpoint::Leaderboards.mix_with_params(params);

        // Ok(self
        //     .inner_connect(endpoint)
        //     .await?
        // )
        Ok(())
    }

    async fn all_map(&self) -> crate::ClientResult<models::AllMap> {
        let endpoint = HashMap::from([("version", "2")]);
        let endpoint = Endpoint::MapRotation.mix_with_params(endpoint);

        Ok(self.inner_connect::<models::AllMap>(&endpoint).await?)
    }

    async fn casual_map(&self) -> crate::ClientResult<models::Map> {
        let endpoint = Endpoint::MapRotation.to_url();

        Ok(self.inner_connect(&endpoint).await?)
    }

    async fn rank_map(&self) -> crate::ClientResult<models::Map> {
        let endpoint = HashMap::from([("version", "2")]);
        let endpoint = Endpoint::MapRotation.mix_with_params(endpoint);

        Ok(self
            .inner_connect::<models::AllMap>(&endpoint)
            .await?
            .ranked)
    }

    async fn other_map(&self) -> crate::ClientResult<models::Map> {
        let endpoint = HashMap::from([("version", "2")]);
        let endpoint = Endpoint::MapRotation.mix_with_params(endpoint);

        Ok(self.inner_connect::<models::AllMap>(&endpoint).await?.other)
    }

    async fn predator(&self) -> crate::ClientResult<models::Rp> {
        let endpoint = Endpoint::Predator.to_url();

        Ok(self
            .inner_connect::<models::predator::Root>(&endpoint)
            .await?
            .rp)
    }

    // #[deprecated(note = "no longer supported")]
    // async fn get_store(&self) -> crate::ClientResult<()> {
    //     Ok(())
    // }

    // async fn craft(&self) -> crate::ClientResult<()> {
    //     Ok(())
    // }

    // async fn news(&self) -> crate::ClientResult<()> {
    //     Ok(())
    // }

    async fn serverstatus(&self) -> crate::ClientResult<models::ServerStatus> {
        let endpoint = Endpoint::ServerStatus.to_url();
        Ok(self
            .inner_connect::<models::ServerStatus>(&endpoint)
            .await?)
    }
}
