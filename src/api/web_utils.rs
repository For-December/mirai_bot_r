use std::{collections::HashMap, process, str::FromStr};

use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    StatusCode,
};

#[derive(Debug, Clone, Default)]
pub struct ApiParam<'a> {
    pub url: &'a str,
    pub json: String,
    pub query: HashMap<&'a str, &'a str>,
    pub headers: HashMap<&'a str, &'a str>,
    pub form: HashMap<&'a str, &'a str>,
}

pub async fn post_utils<'a>(api_param: ApiParam<'a>) -> Result<String, String> {
    let mut header_map = HeaderMap::new();
    api_param.headers.iter().for_each(|(k, v)| {
        header_map.insert(
            HeaderName::from_str(*k).unwrap(),
            HeaderValue::from_str(*v).unwrap(),
        );
    });
    let url = api_param.url;
    let query = api_param.query;
    let json = api_param.json;
    let form = api_param.form;
    let mut builder = reqwest::Client::builder()
        .no_proxy() // 取消系统代理
        // .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
        .post(url)
        .query(&query)
        .body(json.clone())
        .headers(header_map.clone()); // 注意，content type会被form修改

    // 仅当设置了 form 参数时，才生效
    if !form.is_empty() {
        builder = builder.form(&form);
    }

    match builder.send().await {
        Ok(res) => match res.status() {
            StatusCode::OK => {
                let res = res.text().await.unwrap();
                Ok(res)
            }
            code => {
                println!("POST url is {url}");
                println!("headers are {:#?}", header_map);
                println!("Body json is {json}");
                println!("query is {:#?}", query);
                println!("form is {:#?}", form);
                println!("resp body is {}", res.text().await.unwrap_or_default());
                Err(format!("RESPONSE error code: {}", code))
            }
        },
        Err(err) => {
            println!("POST req error: {err}");
            println!("POST url is {url}");
            println!("headers are {:#?}", header_map);
            println!("Body json is {json}");
            println!("query is {:#?}", query);
            println!("form is {:#?}", form);
            Err(String::from(err.to_string()))
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
    let res = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
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
        StatusCode::FOUND => {
            let res = res.text().await.unwrap();
            let re = Regex::new(r"(https://\S*?)[?]").unwrap();
            if let Some(captures) = re.captures(&res) {
                let url = captures.get(1).map_or("", |m| m.as_str());
                // 如果是None则返回""，否则转变为&str并返回
                if url.is_empty() {
                    return Err(String::from("没有匹配到合适的重定向链接"));
                }
                return Ok(String::from(url));
            } else {
                return Err(String::from("没有匹配到重定向链接"));
            }
        }
        code => Err(format!("RESPONSE error code: {}", code)),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        api::web_utils::post_utils,
        setup::conf::{AppConfig, APP_CONF},
    };

    use super::get_utils;

    #[tokio::test]
    async fn test_post() {
        let res = get_utils(
            String::new(),
            // "https://b23.tv/L542xQG",
            "https://b23.tv/5V59Vpv",
            HashMap::new(),
            HashMap::new(),
        )
        .await
        .unwrap();
        println!("{}", res);
    }

    #[tokio::test]
    async fn test_get() {
        let res = get_utils(
            String::new(),
            // "https://b23.tv/L542xQG",
            "https://api.bilibili.com/x/web-interface/index/top/rcmd",
            HashMap::new(),
            vec![("Cookie", APP_CONF.bilibili.cookies.as_str())]
                .into_iter()
                .collect(),
        )
        .await
        .unwrap();
        println!("{}", res);
    }
}
