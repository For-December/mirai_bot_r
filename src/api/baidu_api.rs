use std::{collections::HashMap, process};

use serde_json::Value;

use crate::{api::web_utils::post_utils, setup::conf::APP_CONF};

pub async fn get_access_token() -> String {
    // 查询参数
    let mut query = HashMap::new();
    query.insert("grant_type", "client_credentials");
    query.insert("client_id", &&APP_CONF.wx_api.api_key.as_str());
    query.insert("client_secret", &APP_CONF.wx_api.secret_key.as_str());

    // 请求头
    let mut headers = HashMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded");
    let res = post_utils(
        String::new(),
        "https://aip.baidubce.com/oauth/2.0/token",
        query,
        headers,
    )
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
