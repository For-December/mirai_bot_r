use std::{collections::HashMap, process, str::FromStr};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    StatusCode,
};

pub async fn post_utils(
    json: String,
    url: &str,
    query: HashMap<&str, &str>,
    headers: HashMap<&str, &str>,
) -> Result<String, String> {
    let mut header_map = HeaderMap::new();
    headers.iter().for_each(|(k, v)| {
        header_map.insert(
            HeaderName::from_str(*k).unwrap(),
            HeaderValue::from_str(*v).unwrap(),
        );
    });
    let res = reqwest::Client::new()
        .post(url)
        .query(&query)
        .body(json.clone())
        .headers(header_map.clone())
        .send()
        .await
        .unwrap_or_else(|err| {
            println!("POST req error: {err}");
            println!("POST url is {url}");
            println!("headers are {:#?}", header_map);
            println!("Body json is {json}");
            println!("query is {:#?}", query);
            process::exit(1);
        });

    match res.status() {
        StatusCode::OK => {
            let res = res.text().await.unwrap();
            Ok(res)
        }
        code => Err(format!("RESPONSE error code: {}", code)),
    }

    // println!("{:#?}",req);
    // let res = req.send().await?.text()?;
    // let v: Value = serde_json::from_str(&res)?;
    // println!("{:#?}", v);
    // Ok(res)
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
