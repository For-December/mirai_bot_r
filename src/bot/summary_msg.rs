use std::{
    collections::VecDeque,
    f32::consts::E,
    sync::{Arc, Mutex},
};

use crate::api::wx_chat::{self, Conversation};

use super::{group::GroupSender, message::Message};
use lazy_static::lazy_static;
lazy_static! {
    static ref CONVERSATIONS: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
}
pub fn accumulate_msg(message_chain: &Vec<Message>, sender: &GroupSender) {
    let mut data = String::new();
    data.push_str(sender.get_member_name().as_str());
    data.push_str(": ");

    for ele in message_chain {
        let msg = match ele._type.as_str() {
            "At" => ele
                .clone()
                .display
                .unwrap_or_default()
                .trim_matches('@')
                .to_string(),
            "Plain" => ele.clone().text.unwrap_or_default().to_string(),
            "Image" => String::from("[图片]"),
            _ => String::new(),
        };
        if data.len() > 30 {
            continue;
        }
        data.push_str(&msg);
    }
    println!("data::{}", data);
    let cons = Arc::clone(&CONVERSATIONS);

    let mut cons = cons.lock().unwrap();
    while cons.len() >= 50 {
        cons.pop_front();
    }
    cons.push_back(data);
}
pub fn summary() -> String {
    // return CONVERSATIONS.lock().unwrap().len().to_string();
    let cons = Arc::clone(&CONVERSATIONS);

    let mut data = String::new();
    let cons = cons.lock().unwrap();
    for ele in cons.iter() {
        data.push_str(&ele);
        data.push_str("\n");
    }
    data
}
