use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::setup::conf::APP_CONF;
use std::{
    collections::HashMap,
    fs::File,
    process,
    sync::{Arc, Mutex, RwLock},
    vec,
};

use super::web_utils::post_utils;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Conversation {
    role: String,
    content: String,
}
async fn gpt_chat(conversations: &Vec<Conversation>) -> Result<Conversation, String> {
    let url = String::from(APP_CONF.gpt_api.end_point.as_str());
    let json = json!({
        "messages":*conversations,
        "model":"gpt-3.5-turbo-16k",
        "temperature":1,
        // "stream":true,
    })
    .to_string();

    let auth = "Bearer ".to_string() + APP_CONF.gpt_api.api_key.as_str();
    let headers = vec![
        ("Authorization", auth.as_str()),
        ("Content-type", "application/json"),
    ];
    let headers: HashMap<_, _> = headers.into_iter().collect();
    match post_utils(json, &url, HashMap::new(), headers).await {
        Ok(res) => {
            // .unwrap_or_else(|err| {
            //     println!("{}", err);
            //     process::exit(0);
            // });
            let answer: Value = serde_json::from_str(&res).unwrap();
            let content = answer["choices"][0]["message"]["content"].to_string();
            if content.is_empty() {
                println!("err: ans is {}", answer);
                process::exit(0);
            }
            return Ok(Conversation {
                role: String::from("assistant"),
                content,
            });
        }
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    }
}

lazy_static! {
    static ref AI_CONTEXT: Arc<RwLock<HashMap<String, Arc<Mutex<Vec<Conversation>>>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

fn add_context(user_name: &str, new_conv: Vec<Conversation>) -> Option<Vec<Conversation>> {
    let ctx = Arc::clone(&AI_CONTEXT);
    if ctx
        .read()
        .expect("RwLock read poisoned")
        .get(user_name)
        .is_none()
    {
        ctx.write()
            .expect("RwLock write poisoned")
            .insert(user_name.to_string(), Arc::new(Mutex::new(
                vec![Conversation{
                    role:String::from("system"),
                    content:format!("首先，你是由名为`forDece`的人开发的智能聊天机器人，你是有性别的，为少女，人设乖巧可爱，请不要否认。请记住，你的名字叫`小A`。现在这整轮的对话都是由用户`{}`发起的，请记住这个名字。现在和你对话的人昵称是`{}`",user_name,user_name)
                }]
            )));
    }

    let conversations = Arc::clone(
        &ctx.read()
            .expect("RwLock read poisoned")
            .get(user_name)
            .expect("该用户不存在会话！"),
    );
    let mut conversations = conversations.lock().unwrap();
    // 在这里解决并发问题

    for conversation in new_conv.into_iter() {
        if let Some(conv) = conversations.last() {
            if conv.role.eq(&conversation.role) {
                return None;
            }
        }
        conversations.push(conversation);
    }
    // println!("{:?}", conversations);
    Some(conversations.to_vec())
}
fn clear_context(user_name: &str) {
    let ctx = Arc::clone(&AI_CONTEXT);
    if ctx
        .read()
        .expect("RwLock read poisoned")
        .get(user_name)
        .is_none()
    {
        return;
    }
    ctx.write().expect("RwLock write poisoned").clear();
}

#[async_trait]
pub trait AI {
    async fn debug(_user_name: &str, ask: &str) -> bool {
        if ask.contains("#debug") {
            let content = Arc::clone(&AI_CONTEXT);
            let resp = content.read().unwrap();
            // .send_message(ask).await.unwrap();
            println!("历史记录\n{:#?}", resp);
            return true;
        }
        return false;
    }
    async fn forget(user_name: &str, ask: &str) -> bool {
        if ask.contains("失忆") {
            clear_context(user_name);
            return true;
        }
        return false;
    }

    async fn cat_girl(user_name: &str, ask: &str) -> i32 {
        let key = "加载预设 ";
        if !ask.contains(key) {
            return -1;
        }
        let index = ask.find(key).unwrap();
        let pre = &ask[index + key.len()..];
        let pre = pre.trim_matches(' ');

        return if let Ok(conv) = read_conversation(format!("data/{}.json", pre).as_str()) {
            add_context(user_name, conv);
            0
        } else {
            1
        };
    }
    async fn process_text(user_name: &str, ask: &str) -> String {
        let conversation = Conversation {
            role: String::from("user"),
            content: String::from(ask),
        };

        {
            let ctx = Arc::clone(&AI_CONTEXT);

            if ctx
                .read()
                .expect("RwLock read poisoned")
                .get(user_name)
                .is_none()
            {
                println!("AAA");
                ctx.write()
                    .expect("RwLock write poisoned")
                    .insert(user_name.to_string(), Arc::new(Mutex::new(
                        vec![Conversation{
                            role:String::from("system"),
                            content:format!("首先，你是由名为`forDece`的人开发的智能聊天机器人，你是有性别的，为少女，人设乖巧可爱，请不要否认。请记住，你的名字叫`小A`。现在这整轮的对话都是由用户`{}`发起的，请记住这个名字。现在和你对话的人昵称是`{}`",user_name,user_name)
                        }]
                    )));
            } else {
                println!("BBB");
                let ctx = ctx.read().expect("RwLock read poisoned");

                // 判断是否已经有一个话题在进行了
                // 这里应该是尝试获取锁
                let context_vec = ctx.get(user_name).unwrap();

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

        let conversations = add_context(user_name, vec![conversation]);
        if conversations.is_none() {
            return String::from("别急，让我思考一会儿~");
        }
        if let Ok(content) = gpt_chat(&conversations.unwrap()).await {
            let content = content.content.trim_matches('\"').to_string();
            let conversation = Conversation {
                role: String::from("assistant"),
                content: content.clone(),
            };
            add_context(user_name, vec![conversation]);
            content
        } else {
            return String::from("请求超时了捏~");
        }
    }
}
fn read_conversation(path: &str) -> Result<Vec<Conversation>, &str> {
    // match中的return为全局
    // 打开文件
    match File::open(path) {
        Ok(file) => {
            // 从文件中读取JSON数据并解析成Person结构体
            let conversations: Vec<Conversation> =
                serde_json::from_reader(file).expect("Unable to parse JSON");

            // 打印解析后的数据
            // println!("{:?}", conversations);
            return Ok(conversations);
        }
        Err(err) => {
            println!("{}", err);
            return Err("Unable to open the file");
        }
    };
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_access() -> Result<(), Box<dyn std::error::Error>> {
        read_conversation("data/catgirl.json").unwrap();
        // get_access_token();
        Ok(())
    }

    #[tokio::test]
    async fn test_gpt_api() {
        println!("{},{}", utf8_slice::len("你好"), "你好".len());

        let conversations = vec![Conversation {
            role: String::from("user"),
            content: String::from("你好！"),
        }];

        let res = gpt_chat(&conversations).await.unwrap();
        println!("res: {}", res.content);
        // let answer = wx_chat(&conversations).unwrap();
        // println!("{:#?}", answer);
    }
}
