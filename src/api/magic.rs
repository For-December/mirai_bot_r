use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::web_utils::get_utils;
use std::collections::HashMap;
use std::process;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MagicDetails {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub size: u64,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshots: Option<Vec<Value>>,
}

pub async fn get_preview(url: &str) -> MagicDetails {
    let ans = get_utils(
        String::new(),
        "https://whatslink.info/api/v1/link",
        vec![("url", url)].into_iter().collect(),
        HashMap::new(),
    )
    .await
    .unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(0);
    });
    let ans: MagicDetails = serde_json::from_str(&ans).unwrap();
    ans
}

#[cfg(test)]
pub mod test {

    use super::*;
    #[tokio::test]
    async fn test_get_preview() {
        get_preview("magnet:?xt=urn:btih:AE5C1273F221FD17D42005F725AF0D596B8C5D4D").await;
    }
}
