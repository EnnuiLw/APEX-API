# APEX API with Rust

Author: [EnnuiLw](https://github.com/EnnuiLw)

Last edit: 27/09/25

## [APEX LEGEND STATUS](https://portal.apexlegendsapi.com/)


*I'll write more details on someday


|function|check|
|:---:|:---:|
|Player statistics||
|Match history||
|Leaderboards||
|Map rotation|✓|
|Predator|✓|
|Store||
|Crafting rotation||
|News||
|Server status|✓|
|Origin||
|Name to UID||



# Usage

```rs
use apex_api::{Client, BaseClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = "YOUR_TOKEN";
    let client = Client::new(token);

    let map = client.ranked_map().await?;
}
```
