use crate::{api::wx_chat::AI, setup::conf::APP_CONF};

use super::{
    bot_trait::{BotAction, EventHandler},
    event::Event,
    group::GroupSender,
    message::Message,
    message::MessageChain,
};
use rand::{thread_rng, Rng};
use serde_json::{json, to_value, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct MyBot {
    qq: String,
    session_key: String,
}

impl MyBot {
    pub fn new() -> Result<MyBot, Box<dyn std::error::Error>> {
        let config = &APP_CONF;

        let session_key = Self::get_verify(&config.base_url, &config.verify_key)?;
        let qq = config.bot_qq.clone();
        Self::bind_release_verify(&session_key, &qq, false)?;
        Ok(MyBot { session_key, qq })
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

    fn get_verify(base_url: &str, verify_key: &str) -> Result<String, Box<dyn std::error::Error>> {
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

impl BotAction for MyBot {
    fn send_group_msg(&self, group_num: &str, msg: &MessageChain) {
        let message_chain: Value = to_value(msg.get_message_chain()).unwrap();
        let json = json!({
            "target": group_num,
            "messageChain": message_chain
        })
        .to_string();
        println!("{}", json);

        super::api_utils::post_msg(json, "/sendGroupMessage", &self.session_key);
    }

    fn send_group_nudge(&self, subject: String, target: String) {
        let json = json!({
            "subject":subject,
            "target":target,
            "kind":"Group"
        })
        .to_string();
        super::api_utils::post_msg(json, "/sendNudge", &self.session_key).unwrap();
    }
}
impl AI for MyBot {}
impl EventHandler for MyBot {
    fn handle_group_event(&self, message_chain: &MessageChain, sender: &GroupSender) {
        let message_chain = message_chain.get_message_chain();
        let mut store_chain: Vec<&Message> = Vec::new();
        for ele in message_chain {
            if ele._type.eq_ignore_ascii_case("Source") {
                continue;
            }
            store_chain.push(ele);
        }
        println!("{:#?}", serde_json::to_string(&store_chain).unwrap());

        if !message_chain[1]._type.eq_ignore_ascii_case("Plain")
            || !message_chain[1].text.as_ref().unwrap().eq("测试3")
        {
            if message_chain[1]._type.eq("At")
                && message_chain[1].target.unwrap() == ***REMOVED***
                && message_chain[2]._type.eq("Plain")
            {
                let ans = self.process_text(message_chain[2].text.as_ref().unwrap().as_str());
                let ans = MessageChain::new()
                    .build_at(sender.get_id())
                    .build_text(ans);
                self.send_group_msg(sender.get_group().id.to_string().as_str(), &ans);
            }

            return;
        }

        let msg = MessageChain::new()
            .build_text(String::from("你好！"))
            .build_text(String::from("晚上好！"))
            .build_img(String::from(
                "https://i0.hdslb.com/bfs/album/67fc4e6b417d9c68ef98ba71d5e79505bbad97a1.png",
            ))
            .build_at(String::from(sender.get_id()));
        self.send_group_msg(&sender.get_group().id.to_string(), &msg);
        self.send_group_nudge(sender.get_group().id.to_string(), sender.get_id())
    }

    fn handle_nudge_event(&self, from_id: &String, target: &String, subject: &Value) {
        println!("事件发生!");
        if target.eq("***REMOVED***") {
            if thread_rng().gen_range(0..10) < 6 {
                return;
            }
            let msg = MessageChain::new()
                .build_at(String::from(from_id))
                .build_text(String::from("别戳我！"))
                .build_img(String::from("https://api.vvhan.com/api/acgimg"));
            self.send_group_msg(&subject["id"].to_string(), &msg);
        }
    }
}
