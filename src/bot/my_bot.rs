use crate::{api::wx_chat::AI, setup::conf::APP_CONF};

use super::{event::Event, group::GroupSender, message::Message, message::MessageChain};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MyBot {
    pub qq: String,
    pub session_key: String,
    pub is_mute: bool,
}
unsafe impl Sync for MyBot {}
impl MyBot {
    pub fn new() -> Result<MyBot, Box<dyn std::error::Error>> {
        let config = &APP_CONF;

        let session_key = Self::get_verify(&config.verify_key)?;
        let qq = config.bot_qq.clone();
        Self::bind_release_verify(&session_key, &qq, false)?;
        Ok(MyBot {
            session_key,
            qq,
            is_mute: false,
        })
    }
    pub fn mute(&mut self) {
        self.is_mute = true;
    }
    pub fn active(&mut self) {
        self.is_mute = false;
    }

    pub fn get_events(&self, count: i32) -> Result<Option<Vec<Event>>, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        let count = count.to_string();
        map.insert("count", count.as_str());
        let res = super::api_utils::get_msg(map, "/fetchMessage", &self.session_key)?;
        let res: Value = serde_json::from_str(&res)?;
        let data_arr: Option<Vec<Value>> = serde_json::from_value(res["data"].clone())?;
        if data_arr.is_none() {
            return Ok(None);
        }

        let data_arr = data_arr.unwrap();
        let mut res_arr = Vec::<Event>::new();
        for data in data_arr {
            if data["type"].to_string().contains("GroupMessage") {
                let message_chain: Vec<Message> =
                    serde_json::from_value(data["messageChain"].clone()).unwrap();
                res_arr.push(Event::GroupEvent((
                    MessageChain::from(message_chain),
                    GroupSender::new(data["sender"].clone()),
                )));
            } else if data["type"].to_string().contains("NudgeEvent") {
                // println!("事件");
                res_arr.push(Event::NudgeEvent((
                    data["fromId"].to_string(),
                    data["target"].to_string(),
                    data["subject"].clone(),
                )))
            } else {
                println!("event:: {}", data["type"].to_string());
            }
        }
        return Ok(Some(res_arr));

        // println!("{:#?}", data);
        // Ok(None)
    }

    fn get_verify(verify_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let json = json!({
            "verifyKey":verify_key,
        })
        .to_string();
        let res = super::api_utils::post_msg(json, "/verify", "")?;
        let v: Value = serde_json::from_str(&res)?;
        let res: String = v["session"].to_string().trim_matches('"').to_string();
        Ok(res)
    }

    fn bind_release_verify(
        session_key: &str,
        qq: &str,
        is_release: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let json = json!({
            "sessionKey":session_key,
            "qq":qq
        })
        .to_string();
        let res = match is_release {
            true => super::api_utils::post_msg(json, "/release", "")?,
            false => super::api_utils::post_msg(json, "/bind", "")?,
        };
        let v: Value = serde_json::from_str(&res)?;
        let res: String = v["msg"].to_string();
        Ok(res)
    }
}
// 实现 Drop trait
impl Drop for MyBot {
    fn drop(&mut self) {
        // 在这里执行资源的清理操作，例如关闭文件、释放内存等
        println!("Dropping MyBot...");
        Self::bind_release_verify(&self.session_key, &self.qq, true).unwrap();

        // 在 drop 方法中执行异步清理操作
        // tokio::task::spawn_blocking(|| {
        // code async
        // });
    }
}

impl AI for MyBot {}
