use async_trait::async_trait;
use serde_json::Value;

use super::{group::GroupSender, message::MessageChain};
pub trait EventHandler {
    fn handle_group_event(&'static self, message_chain: &MessageChain, sender: &GroupSender);
    fn handle_nudge_event(&'static self, from_id: &String, target: &String, subject: &Value);
}

#[async_trait]
pub trait BotAction {
    async fn send_group_msg(&'static self, group_num: &str, msg: &MessageChain);
    async fn send_group_nudge(&'static self, subject: String, target: String);
}

#[async_trait]
pub trait GroupAdmin {
    async fn member_admin(&'static self, group_num: &str, member_id: &str, assign: bool) -> String;
}
