#[cfg(test)]
mod tests {
    use apex_api::BaseClient;

    #[tokio::test]
    async fn map_works() {
        let client = apex_api::Client::from_env().unwrap();
        assert!(client.all_map().await.is_ok());
        // cooltime
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        assert!(client.casual_map().await.is_ok());
        assert!(client.rank_map().await.is_ok());
        assert!(client.other_map().await.is_ok());

        // let maps = client.all_map().await.unwrap();
        // let maps = vec![
        //     maps.battle_royale,
        //     maps.ranked,
        //     maps.other,
        // ]
        //     .into_iter()
        //     .map(|m| format!("Now:\t{}\nNext:\t{}\n",
        //         &m.current.map, &m.next.map))
        //     .collect::<Vec<_>>()
        //     .join("\n");
        // println!("{maps}");
    }

    #[tokio::test]
    async fn serverstatus_works() {
        let client = apex_api::Client::from_env().unwrap();

        let server_status = client.serverstatus().await;
        assert!(server_status.is_ok());
    }

    #[tokio::test]
    async fn predator_works() {
        let client = apex_api::Client::from_env().unwrap();

        let server_status = client.predator().await;
        assert!(server_status.is_ok());
    }

    #[tokio::test]
    async fn leaderboards_works() {
        // let client = apex_api::Client::from_env().unwrap();

        // let server_status = client.predator().await;
        // assert!(server_status.is_ok());
    }

    #[tokio::test]
    async fn player_works() {}
}
