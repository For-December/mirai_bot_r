use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::setup::conf::APP_CONF;
use std::collections::HashMap;

use super::web_utils::post_utils;

fn get_access_token() -> String {
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
    .unwrap();
    let res: Value = serde_json::from_str(&res).unwrap();
    let res = res["access_token"]
        .to_string()
        .trim_matches(|c| c == '\"')
        .to_string();
    println!("{}", res);
    return res;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Conversation {
    role: String,
    content: String,
}
fn wx_chat(conversations: &Vec<Conversation>) -> Result<Conversation, Box<dyn std::error::Error>> {
    let url = String::from("https://aip.baidubce.com/")
        + "rpc/2.0/ai_custom/v1/wenxinworkshop/chat/eb-instant?"
        + "access_token="
        + get_access_token().as_str();
    let json = json!({
        "messages":*conversations
    })
    .to_string();
    let res = post_utils(json, &url, HashMap::new(), HashMap::new()).unwrap();
    let answer: Value = serde_json::from_str(&res)?;
    let content = answer["result"].to_string();
    return Ok(Conversation {
        role: String::from("assistant"),
        content,
    });
}
pub trait AI {
    fn process_text(&self, ask: &str) -> String {
        let conversations = vec![Conversation {
            role: String::from("user"),
            content: String::from(ask),
        }];
        wx_chat(&conversations)
            .unwrap()
            .content
            .trim_matches('\"')
            .to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_access() -> Result<(), Box<dyn std::error::Error>> {
        get_access_token();
        Ok(())
    }

    #[test]
    fn test_wx_api() {
        let conversations = vec![Conversation {
            role: String::from("user"),
            content: String::from("你好！"),
        }];
        let answer = wx_chat(&conversations).unwrap();
        println!("{:#?}", answer);
    }
}
