use serde_json::Value;

use super::{group::GroupSender, message::MessageChain};
pub trait EventHandler {
    fn handle_group_event(&self, message_chain: &MessageChain, sender: &GroupSender);
    fn handle_nudge_event(&self, from_id: &String, target: &String, subject: &Value);
}

pub trait BotAction {
    fn send_group_msg(&self, group_num: &str, msg: &MessageChain);
    fn send_group_nudge(&self, subject: String, target: String);
}

pub trait GroupAdmin {
    fn member_admin(&self, group_num: &str, member_id: &str, assign: bool) -> String;
}
