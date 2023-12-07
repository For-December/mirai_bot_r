use std::process;

use async_trait::async_trait;
use regex::Regex;
use serde_json::{json, to_value, Value};

use crate::LAST_MSG;

use super::{api_utils, bot_trait::BotAction, message::MessageChain, my_bot::MyBot};

#[async_trait]
impl BotAction for MyBot {
    async fn recall_last_group_msg(&'static self, subject: String) {
        let last_msg = LAST_MSG.clone().lock().unwrap().pop();
        // 及时释放锁
        tokio::task::spawn(async move {
            match last_msg {
                Some(message_id) => {
                    let json = json!({
                            "target": subject,
                            "messageId": message_id,
                    })
                    .to_string();
                    api_utils::post_msg(json, "/recall", &self.session_key)
                        .await
                        .unwrap();
                }
                None => {
                    println!("无消息可撤回");
                }
            }
        });
    }
    async fn send_group_msg(&'static self, msg: &MessageChain) -> Result<String, String> {
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
        let pattern = Regex::new(r#""base64":"\S+""#).unwrap();
        println!(
            "sendToGroup: {}",
            pattern.replace(&json, r#""base64":"编码后的数据""#)
        );
        // println!("{}", json);

        match super::api_utils::post_msg(json, "/sendGroupMessage", &self.session_key).await {
            Ok(msg) => {
                println!("send msg success: {}", msg);
                let msg: Value = serde_json::from_str(&msg).unwrap();
                Ok(msg["messageId"].to_string())
            }
            Err(err) => Err(format!("send group msg error: {}", err)),
        }
    }

    async fn send_group_nudge(&'static self, subject: String, target: String) {
        let json = json!({
            "subject":subject,
            "target":target,
            "kind":"Group"
        })
        .to_string();
        super::api_utils::post_msg(json, "/sendNudge", &self.session_key)
            .await
            .unwrap();
    }
}
