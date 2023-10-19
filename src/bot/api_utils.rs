use reqwest::StatusCode;

use crate::setup::conf::APP_CONF;
use std::{collections::HashMap, process};
pub async fn post_msg(json: String, api_path: &str, session_key: &str) -> Result<String, String> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let res = reqwest::Client::new()
        .post(&(APP_CONF.base_url.clone() + api_path))
        .body(json)
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
            Ok(res)
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
