use std::process;

use async_trait::async_trait;
use serde_json::{json, to_value, Value};

use super::{bot_trait::BotAction, message::MessageChain, my_bot::MyBot};

#[async_trait]
impl BotAction for MyBot {
    async fn send_group_msg(&'static self, msg: &MessageChain) {
        let group_num = msg
            .group_num
            .as_ref()
            .unwrap_or_else(|| {
                println!("未包含群号字段，发送群消息失败！");
                process::exit(0);
            })
            .as_str();
        let message_chain: Value = to_value(msg.get_message_chain()).unwrap();
        let json = json!({
            "target": group_num,
            "messageChain": message_chain
        })
        .to_string();
        println!("{}", json);

        super::api_utils::post_msg(json, "/sendGroupMessage", &self.session_key)
            .await
            .unwrap();
    }

    async fn send_group_nudge(&'static self, subject: String, target: String) {
        let json = json!({
            "subject":subject,
            "target":target,
            "kind":"Group"
        })
        .to_string();
        super::api_utils::post_msg(json, "/sendNudge", &self.session_key).await.unwrap();
    }
}
