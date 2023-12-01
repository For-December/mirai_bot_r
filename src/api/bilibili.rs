use std::collections::HashMap;

use regex::Regex;
use serde_json::Value;

use crate::api::web_utils::{get_utils, post_utils};

pub async fn get_video_summary(bili_url: String) -> String {
    match post_utils(
        bili_url,
        "http://localhost:9876/ai/",
        HashMap::new(),
        HashMap::new(),
    )
    .await
    {
        Ok(resp) => resp.replace("\\n", "\n"),
        Err(err) => {
            format!("error: {}", err)
        }
    }
}

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

pub struct BVInfo {
    pub desc: String,
    pub pic: String,
    pub title: String,
    pub owner_name: String,
    pub url: String,
}
pub async fn get_info(text: &str) -> Result<BVInfo, String> {
    if text.contains("bilibili.com/video/") {
        // 直接获取bv
        return Ok(get_bv_info(get_bv(text)?).await);
    }
    if text.contains("b23.tv/") {
        // 间接获取bv
        let re = Regex::new(r"(https://b23.tv/\S+)[?/]?").unwrap();
        if let Some(captures) = re.captures(&text) {
            let url = captures.get(1).map_or("", |m| m.as_str());
            // 如果是None则返回""，否则转变为&str并返回
            if url.is_empty() {
                return Err(String::from("未找到合适的链接"));
            }

            println!("{}", url);
            // 解包bv url
            let url = get_utils(String::new(), url, HashMap::new(), HashMap::new())
                .await
                .unwrap();
            println!("url is=> {}", url);
            return Ok(get_bv_info(get_bv(&url)?).await);
        } else {
            return Err(String::from("未匹配到任何链接"));
        }
    }

    return Err(String::from("未找到BV号"));
}
fn get_bv<'a>(url: &'a str) -> Result<&'a str, String> {
    match url.find("BV") {
        Some(index) => {
            let temp = &url[index..];
            let end = temp
                .find("?")
                .unwrap_or(temp.find("/").unwrap_or(temp.len()));
            Ok(&temp[..end])
        }
        None => Err(String::from("未匹配到bv号")),
    }
}
pub async fn get_bv_info(bvid: &str) -> BVInfo {
    let url = "https://api.bilibili.com/x/web-interface/view";
    let res = get_utils(
        String::new(),
        url,
        vec![("bvid", bvid)].into_iter().collect(),
        HashMap::new(),
    )
    .await
    .unwrap();

    let res: Value = serde_json::from_str(&res).unwrap();

    let desc = res["data"]["desc"]
        .to_string()
        .replace("\\n", "\n")
        .replace("\\", "");
    let pic = res["data"]["pic"].to_string().trim_matches('"').to_string();
    let title = res["data"]["title"].to_string();
    let owner_name = res["data"]["owner"]["name"].to_string();
    let url = format!("https://www.bilibili.com/video/{}/", bvid);
    // let desc = res["data"]["desc"]
    BVInfo {
        desc,
        pic,
        title,
        owner_name,
        url,
    }
    // println!("{}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    pub async fn test_anime() {
        get_latest_anime().await;
    }

    #[tokio::test]
    pub async fn test_info() {
        get_bv_info("BV1Dg4y1973e").await;
        let res =
            get_video_summary(String::from("https://www.bilibili.com/video/BV1dw411B7BY/")).await;
        println!("{}", res);
    }
}
