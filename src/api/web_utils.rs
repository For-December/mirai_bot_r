use std::{collections::HashMap, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};

pub fn post_utils(
    json: String,
    url: &str,
    query: HashMap<&str, &str>,
    headers: HashMap<&str, &str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut header_map = HeaderMap::new();
    headers.iter().for_each(|(k, v)| {
        header_map.insert(
            HeaderName::from_str(*k).unwrap(),
            HeaderValue::from_str(*v).unwrap(),
        );
    });
    let req = reqwest::blocking::Client::new()
        .post(url)
        .query(&query)
        .body(json)
        .headers(header_map);

    // println!("{:#?}",req);
    let res = req.send()?.text()?;
    // let v: Value = serde_json::from_str(&res)?;
    // println!("{:#?}", v);
    Ok(res)
}

// pub fn get_utils(
//     map: HashMap<&str, &str>,
//     api_path: &str,
//     session_key: &str,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     println!("{}", APP_CONF.base_url.clone() + api_path);
//     let mut req_builder = reqwest::blocking::Client::new()
//         .get(&(APP_CONF.base_url.clone() + api_path))
//         .header("sessionKey", session_key);
//     for ele in map {
//         req_builder = req_builder.query(&[ele]);
//     }
//     let res = req_builder.send()?.text()?;
//     println!("{:#?}", res);

//     Ok(res)
// }
