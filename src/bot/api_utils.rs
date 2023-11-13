use reqwest::StatusCode;
use serde_json::Value;

use crate::setup::conf::APP_CONF;
use std::{collections::HashMap, process};
pub async fn post_msg(json: String, api_path: &str, session_key: &str) -> Result<String, String> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let res = reqwest::Client::new()
        .post(&(APP_CONF.base_url.clone() + api_path))
        .body(json.clone())
        .header("sessionKey", session_key)
        .send()
        .await
        .unwrap_or_else(|err| {
            println!("POST request error: {err}");
            println!("POST url is {}", APP_CONF.base_url.clone() + api_path);
            process::exit(0);
        });

    match res.status() {
        StatusCode::OK => {
            let res = res.text().await.unwrap();
            let resp_json: Value = serde_json::from_str(&res).unwrap();
            if resp_json["code"].to_string().eq("200") {
                Ok(res)
            } else {
                println!("{}", json);
                Err(format!("error: {}", resp_json["msg"].to_string()))
            }
        }
        code => Err(format!("RESPONSE error code: {}", code)),
    }
    // println!("{:#?}", res);
}

pub async fn get_msg(
    map: HashMap<&str, &str>,
    api_path: &str,
    session_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let mut req_builder = reqwest::Client::new()
        .get(&(APP_CONF.base_url.clone() + api_path))
        .header("sessionKey", session_key);
    for ele in map {
        req_builder = req_builder.query(&[ele]);
    }
    let res = req_builder.send().await?.text().await?;
    // println!("{:#?}", res);

    Ok(res)
}
