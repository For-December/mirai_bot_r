use async_trait::async_trait;
use serde_json::{json, to_value, Value};

use super::{bot_trait::BotAction, message::MessageChain, my_bot::MyBot};

#[async_trait]
impl BotAction for MyBot {
    async fn send_group_msg(&'static self, group_num: &str, msg: &MessageChain) {
        let message_chain: Value = to_value(msg.get_message_chain()).unwrap();
        let json = json!({
            "target": group_num,
            "messageChain": message_chain
        })
        .to_string();
        println!("{}", json);

        // super::api_utils::post_msg(json, "/sendGroupMessage", &self.session_key).unwrap();
    }

    async fn send_group_nudge(&'static self, subject: String, target: String) {
        let json = json!({
            "subject":subject,
            "target":target,
            "kind":"Group"
        })
        .to_string();
        super::api_utils::post_msg(json, "/sendNudge", &self.session_key).await;
    }
}
