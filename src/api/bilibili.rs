use std::collections::HashMap;

use serde_json::Value;

use super::web_utils::get_utils;

pub async fn get_latest_anime() -> Vec<(String, String, String)> {
    let url = "https://api.bilibili.com/pgc/web/timeline/v2";
    let mut query = HashMap::new();
    query.insert("season_type", "1");
    query.insert("day_before", "1");
    query.insert("day_after", "5");
    let res = get_utils(String::new(), url, query, HashMap::new())
        .await
        .unwrap();

    let res: Value = serde_json::from_str(&res).unwrap();
    let animes: Vec<Value> = serde_json::from_str(&res["result"]["latest"].to_string()).unwrap();

    let mut animes_tuple = Vec::new();
    for ele in animes {
        animes_tuple.push((
            ele["title"].to_string(),
            ele["pub_index"].to_string(),
            ele["cover"].to_string(),
        ));
    }

    return animes_tuple;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    pub async fn test_anime() {
        get_latest_anime().await;
    }
}
