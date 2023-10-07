use crate::setup::conf::APP_CONF;
use std::collections::HashMap;
pub fn post_msg(
    json: String,
    api_path: &str,
    session_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", APP_CONF.base_url.clone() + api_path);
    let res = reqwest::blocking::Client::new()
        .post(&(APP_CONF.base_url.clone() + api_path))
        .body(json)
        .header("sessionKey", session_key)
        .send()?
        .text()?;
    println!("{:#?}", res);
    Ok(res)
}

pub fn get_msg(
    map: HashMap<&str, &str>,
    api_path: &str,
    session_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", APP_CONF.base_url.clone() + api_path);
    let mut req_builder = reqwest::blocking::Client::new()
        .get(&(APP_CONF.base_url.clone() + api_path))
        .header("sessionKey", session_key);
    for ele in map {
        req_builder = req_builder.query(&[ele]);
    }
    let res = req_builder.send()?.text()?;
    println!("{:#?}", res);

    Ok(res)
}
