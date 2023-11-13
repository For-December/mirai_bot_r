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
        code => {
            println!("POST url is {url}");
            println!("headers are {:#?}", header_map);
            println!("Body json is {json}");
            println!("query is {:#?}", query);
            println!("resp body is {}", res.text().await.unwrap_or_default());
            Err(format!("RESPONSE error code: {}", code))
        }
    }

    // println!("{:#?}",req);
    // let res = req.send().await?.text()?;
    // let v: Value = serde_json::from_str(&res)?;
    // println!("{:#?}", v);
    // Ok(res)
}

pub async fn get_utils(
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
        .get(url)
        .query(&query)
        .body(json.clone())
        .headers(header_map.clone())
        .send()
        .await
        .unwrap_or_else(|err| {
            println!("GET req error: {err}");
            println!("GET url is {url}");
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
}
