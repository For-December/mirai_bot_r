use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};
use std::collections::HashMap;
use std::f32::consts::E;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct MyBot {
    qq: String,
    session_key: String,
    base_url: String,
}

#[derive(Debug, serde::Deserialize)]
struct AppConfig {
    base_url: String,
    verify_key: String,
    bot_qq: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub _type: String, // [Plain, Image, Source]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

enum MessageType {
    Plain(String),
    Image(String),
    Source((i64, i64)),
    At(String),
}
trait With<T> {
    fn with(value: T) -> Self;
}
impl With<MessageType> for Message {
    fn with(value: MessageType) -> Self {
        match value {
            MessageType::Plain(text) => Message {
                _type: String::from("Plain"),
                text: Some(text),
                ..Default::default()
            },
            MessageType::Image(url) => Message {
                _type: String::from("Image"),
                url: Some(url),
                ..Default::default()
            },
            MessageType::Source((id, time)) => Message {
                _type: String::from("Source"),
                id: Some(id),
                time: Some(time),
                ..Default::default()
            },
            MessageType::At(target) => Message {
                _type: String::from("At"),
                target: Some(target),
                ..Default::default()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub permission: String,
}

pub struct GroupSender {
    sender: Value, // messageChain and sender
}
impl GroupSender {
    pub fn new(sender: Value) -> GroupSender {
        GroupSender { sender }
    }
    pub fn get_id(&self) -> String {
        self.sender["id"].to_string()
    }
    pub fn get_member_name(&self) -> String {
        self.sender["memberName"].to_string()
    }
    pub fn get_special_title(&self) -> String {
        self.sender["specialTitle"].to_string()
    }
    pub fn get_group(&self) -> Group {
        println!("{:#?}", self.sender);
        let res: Group = serde_json::from_value(self.sender["group"].clone()).unwrap();
        res
    }
}

pub enum Event {
    GroupEvent((MessageChain, GroupSender)),
}

#[derive(Debug)]
pub struct MessageChain {
    message_chain: Vec<Message>,
}
impl MessageChain {
    // 链式调用，所有权转移
    pub fn new() -> MessageChain {
        let message_chain: Vec<Message> = Vec::new();
        return MessageChain { message_chain };
    }
    pub fn from(message_chain: Vec<Message>) -> MessageChain {
        return MessageChain { message_chain };
    }
    pub fn build_img(mut self, url: String) -> Self {
        // Message::with(String::from("value"));
        self.message_chain
            .push(Message::with(MessageType::Image(url)));

        self
    }
    pub fn build_text(mut self, text: String) -> Self {
        self.message_chain
            .push(Message::with(MessageType::Plain(text)));
        self
    }
    pub fn build_at(mut self, target: String) -> Self {
        self.message_chain
            .push(Message::with(MessageType::At(target)));
        self
    }

    pub fn get_message_chain(&self) -> &Vec<Message> {
        &self.message_chain
    }
}

impl MyBot {
    pub fn new() -> Result<MyBot, Box<dyn std::error::Error>> {
        Self::from_conf("config.yaml")
    }
    pub fn from_conf(file_path: &str) -> Result<MyBot, Box<dyn std::error::Error>> {
        let mut config_file = File::open(file_path)?;
        let mut config_yaml = String::new();

        // 读取配置文件内容
        config_file.read_to_string(&mut config_yaml)?;

        // 解析 YAML 配置文件
        let config: AppConfig = serde_yaml::from_str(&config_yaml)?;
        println!("{:?}", config);

        let session_key = Self::get_verify(&config.base_url, &config.verify_key)?;
        let qq = config.bot_qq;
        let base_url = config.base_url;
        Self::bind_release_verify(&base_url, &session_key, &qq, false)?;
        Ok(MyBot {
            session_key,
            qq,
            base_url,
        })
    }

    pub fn handle_group_event(&self, message_chain: &MessageChain, sender: &GroupSender) {
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
            || !message_chain[1]
                .text
                .as_ref()
                .unwrap()
                .eq_ignore_ascii_case("测试3")
        {
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
    }

    fn post_msg(&self, json: String, url: String) -> Result<String, Box<dyn std::error::Error>> {
        println!("{}", url);
        let res = reqwest::blocking::Client::new()
            .post(url)
            .body(json)
            .header("sessionKey", &self.session_key)
            .send()?
            .text()?;
        println!("{:#?}", res);
        Ok(res)
    }
    fn get_msg(
        &self,
        map: HashMap<&str, &str>,
        url: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("{}", url);
        let mut req_builder = reqwest::blocking::Client::new()
            .get(url)
            .header("sessionKey", &self.session_key);
        for ele in map {
            req_builder = req_builder.query(&[ele]);
        }
        let res = req_builder.send()?.text()?;
        println!("{:#?}", res);

        Ok(res)
    }
    pub fn get_events(&self, count: i32) -> Result<Option<Vec<Event>>, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        let count = count.to_string();
        map.insert("count", count.as_str());
        let res = self.get_msg(map, self.base_url.to_string() + "/fetchMessage")?;
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
            }
        }
        return Ok(Some(res_arr));

        // println!("{:#?}", data);
        // Ok(None)
    }

    pub fn send_group_msg(
        &self,
        group_num: &str,
        msg: &MessageChain,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let message_chain: Value = to_value(msg.get_message_chain())?;
        let json = json!({
            "target": group_num,
            "messageChain": message_chain
        })
        .to_string();
        println!("{}", json);

        self.post_msg(json, self.base_url.to_string() + "/sendGroupMessage")?;
        Ok(())
    }

    fn blocking_post_msg(
        map: HashMap<&str, &str>,
        url: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("{}", url);
        let res = reqwest::blocking::Client::new()
            .post(url)
            .json(&map)
            .send()?
            .text()?;
        println!("{:#?}", res);
        Ok(res)
    }

    fn get_verify(base_url: &str, verify_key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("verifyKey", verify_key);
        let res = Self::blocking_post_msg(map, base_url.to_string() + "/verify")?;
        let v: Value = serde_json::from_str(&res)?;
        let res: String = v["session"].to_string().trim_matches('"').to_string();
        Ok(res)
    }

    fn bind_release_verify(
        base_url: &str,
        session_key: &str,
        qq: &str,
        is_release: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("sessionKey", session_key);
        map.insert("qq", qq);
        let res = match is_release {
            true => Self::blocking_post_msg(map, base_url.to_string() + "/release")?,
            false => Self::blocking_post_msg(map, base_url.to_string() + "/bind")?,
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
        Self::bind_release_verify(&self.base_url, &self.session_key, &self.qq, true).unwrap();

        // 在 drop 方法中执行异步清理操作
        // tokio::task::spawn_blocking(|| {
        // code async
        // });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_msg() -> Result<(), Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");

        // _post_msg(map, String::from("http://httpbin.org/post")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_verify() -> Result<(), Box<dyn std::error::Error>> {
        // let res = get_verify().await?;
        Ok(())
    }
}
