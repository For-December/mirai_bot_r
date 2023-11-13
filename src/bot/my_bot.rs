use crate::setup::conf::APP_CONF;

use super::{event::Event, group::GroupSender, message::Message, message::MessageChain};
use serde_json::{json, Value};
use std::{collections::HashMap, process};

#[derive(Debug)]
pub struct MyBot {
    pub qq: String,
    pub session_key: String,
}

impl MyBot {
    pub async fn build() -> Result<MyBot, Box<dyn std::error::Error>> {
        let session_key = Self::get_verify(APP_CONF.verify_key.as_str())
            .await
            .unwrap_or_else(|err| {
                println!("get verify error: {err}");
                process::exit(0);
            });
        let qq = APP_CONF.bot_qq.clone();

        Self::bind_release_verify(&session_key, &qq, false).await?;
        return Ok(MyBot { qq, session_key });
    }

    pub async fn get_events(
        &'static self,
        count: i32,
    ) -> Result<Option<Vec<Event>>, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        let count = count.to_string();
        map.insert("count", count.as_str());
        let res = super::api_utils::get_msg(map, "/fetchMessage", &self.session_key).await?;
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
                    MessageChain::from(None, message_chain),
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

    async fn get_verify(verify_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let json = json!({
            "verifyKey":verify_key,
        })
        .to_string();
        let res = super::api_utils::post_msg(json, "/verify", "").await?; // 内部处理
        let v: Value = serde_json::from_str(&res).unwrap_or_else(|err| {
            println!("serde json error: {err}");
            println!("not a valid json");
            process::exit(1);
        });
        let res: String = v["session"].to_string().trim_matches('"').to_string();
        Ok(res)
    }

    async fn bind_release_verify(
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
            true => super::api_utils::post_msg(json, "/release", "").await?,
            false => super::api_utils::post_msg(json, "/bind", "").await?,
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

        // 在 drop 方法中执行异步清理操作
        let session_key = self.session_key.clone();
        let qq = self.qq.clone();
        tokio::task::spawn_blocking(|| async move {
            // code async
            Self::bind_release_verify(&session_key, &qq, true)
                .await
                .unwrap();
        });
    }
}
