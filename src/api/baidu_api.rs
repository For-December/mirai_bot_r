use std::{collections::HashMap, process};

use serde_json::Value;

use crate::api::web_utils::{post_utils, ApiParam};

pub async fn get_access_token(api_key: &str, secret_key: &str) -> String {
    // 查询参数
    let mut query = HashMap::new();
    query.insert("grant_type", "client_credentials");
    query.insert("client_id", api_key);
    query.insert("client_secret", secret_key);

    // 请求头
    let mut headers = HashMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded");
    let res = post_utils(ApiParam {
        url: "https://aip.baidubce.com/oauth/2.0/token",
        query,
        headers,
        ..Default::default()
    })
    .await
    .unwrap_or_else(|err| {
        println!("get access token error: {err}");
        process::exit(0);
    });
    let res: Value = serde_json::from_str(&res).unwrap();
    let res = res["access_token"]
        .to_string()
        .trim_matches(|c| c == '\"')
        .to_string();
    println!("{}", res);
    return res;
}
