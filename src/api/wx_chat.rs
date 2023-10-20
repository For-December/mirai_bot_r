use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::setup::conf::APP_CONF;
use std::{
    collections::HashMap,
    process,
    sync::{Arc, Mutex, RwLock},
};

use super::web_utils::post_utils;
use lazy_static::lazy_static;

async fn get_access_token() -> String {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conversation {
    role: String,
    content: String,
}
async fn wx_chat(
    conversations: &Vec<Conversation>,
) -> Result<Conversation, Box<dyn std::error::Error>> {
    let url = String::from("https://aip.baidubce.com/")
        + "rpc/2.0/ai_custom/v1/wenxinworkshop/chat/eb-instant?"
        + "access_token="
        + get_access_token().await.as_str();
    let json = json!({
        "messages":*conversations
    })
    .to_string();
    let res = post_utils(json, &url, HashMap::new(), HashMap::new()).await?;
    let answer: Value = serde_json::from_str(&res)?;
    let content = answer["result"].to_string();
    if content.is_empty() {
        println!("{}", answer);
    }
    return Ok(Conversation {
        role: String::from("assistant"),
        content,
    });
}

lazy_static! {
    static ref AI_CONTEXT: Arc<RwLock<HashMap<String, Arc<Mutex<Vec<Conversation>>>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

fn add_context(user_id: &str, conversation: Conversation) -> Vec<Conversation> {
    // let mut conversations = AI_CONTEXT.lock().as_mut().unwrap();
    // conversations.push(conversation);
    let ctx = Arc::clone(&AI_CONTEXT);
    if ctx
        .read()
        .expect("RwLock read poisoned")
        .get(user_id)
        .is_none()
    {
        ctx.write()
            .expect("RwLock write poisoned")
            .insert(user_id.to_string(), Arc::new(Mutex::new(Vec::new())));
    }

    let conversations = Arc::clone(
        &ctx.read()
            .expect("RwLock read poisoned")
            .get(user_id)
            .expect("该用户不存在会话！"),
    );
    let mut conversations = conversations.lock().unwrap();
    conversations.push(conversation);
    // println!("{:?}", conversations);
    conversations.to_vec()
}
#[async_trait]
pub trait AI {
    async fn process_text(user_id: &str, ask: &str) -> String {
        let conversation = Conversation {
            role: String::from("user"),
            content: String::from(ask),
        };

        {
            let ctx = Arc::clone(&AI_CONTEXT);

            if ctx
                .read()
                .expect("RwLock read poisoned")
                .get(user_id)
                .is_none()
            {
                println!("AAA");
                ctx.write()
                    .expect("RwLock write poisoned")
                    .insert(user_id.to_string(), Arc::new(Mutex::new(Vec::new())));
            } else {
                println!("BBB");
                let ctx = ctx.read().expect("RwLock read poisoned");
                // 判断是否已经有一个话题在进行了
                let context_vec = ctx.get(user_id).unwrap();

                // 下面两行如果连到一起，锁的生命周期只有一行代码，无法保证并发
                // 分离开来，mutex变量会保护vec直到其作用域结束
                let mutex = context_vec.lock().unwrap();
                let role = mutex.last().unwrap().role.clone();
                println!("{}", role);
                if role.contains("user") {
                    return String::from("别急，让我思考一会儿~");
                }
            }
        }

        let conversations = add_context(user_id, conversation);
        let content = wx_chat(&conversations)
            .await
            .unwrap()
            .content
            .trim_matches('\"')
            .to_string();
        let conversation = Conversation {
            role: String::from("assistant"),
            content: content.clone(),
        };
        add_context(user_id, conversation);
        content
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_access() -> Result<(), Box<dyn std::error::Error>> {
        // get_access_token();
        Ok(())
    }

    #[test]
    fn test_wx_api() {
        println!("{},{}", utf8_slice::len("你好"), "你好".len());
        // let conversations = vec![Conversation {
        //     role: String::from("user"),
        //     content: String::from("你好！"),
        // }];
        // let answer = wx_chat(&conversations).unwrap();
        // println!("{:#?}", answer);
    }
}
