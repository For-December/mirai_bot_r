use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use crate::{
    bot::api_utils,
    MY_BOT,
};

use super::{group::GroupSender, message::Message};
use lazy_static::lazy_static;
use log::info;
use serde_json::Value;
lazy_static! {
    static ref CONVERSATIONS: Arc<Mutex<HashMap<String, VecDeque<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}
pub async fn accumulate_msg(message_chain: Vec<Message>, sender: GroupSender) {
    let mut data = String::new();
    data.push_str(sender.get_member_name().as_str());
    data.push_str(": ");
    for ele in message_chain {
        let msg = match ele._type.as_str() {
            "At" => {
                let member_info = api_utils::get_msg(
                    vec![
                        ("target", sender.get_group().id.to_string().as_str()),
                        ("memberId", ele.target.unwrap().to_string().as_str()),
                    ]
                    .into_iter()
                    .collect(),
                    "/memberInfo",
                    &MY_BOT.get().unwrap().session_key,
                )
                .await
                .unwrap();
                let member_info: Value = serde_json::from_str(&member_info).unwrap();
                let name = member_info["memberName"].to_string();

                let at_name = ele
                    .display
                    .unwrap_or(String::from("前面那位用户，"))
                    .to_string();
                if at_name.is_empty() {
                    format!("`@{}`, ", name.trim_matches('\"'))
                } else {
                    at_name
                }
            }
            "Plain" => ele.text.unwrap_or_default().to_string(),
            "Image" => String::from("[图片]"),
            _ => String::new(),
        };
        if utf8_slice::len(&msg) > 200 {
            continue;
        }
        data.push_str(&msg);
    }
    info!("{} => {}", sender.get_group().name, data);
    let cons_map = Arc::clone(&CONVERSATIONS);

    let mut cons_map = cons_map.lock().unwrap();
    let group_num = sender.get_group().id.to_string();
    if !cons_map.contains_key(group_num.as_str()) {
        cons_map.insert(group_num.clone(), VecDeque::new());
    }
    let cons = cons_map
        .get_mut(group_num.as_str())
        .expect("获取map对应值出错");

    while cons.len() >= 150 {
        cons.pop_front();
    }
    cons.push_back(data);
}
pub fn summary(group_num: &str) -> String {
    // return CONVERSATIONS.lock().unwrap().len().to_string();
    let cons_map = Arc::clone(&CONVERSATIONS);
    let cons_map = cons_map.lock().unwrap();
    let cons = cons_map.get(group_num);
    if cons.is_none() {
        return String::from("未收集到数据");
    }
    let cons = cons.unwrap();
    let mut data = String::new();

    for ele in cons.iter() {
        data.push_str(&ele);
        data.push_str("\n");
    }
    data
}
