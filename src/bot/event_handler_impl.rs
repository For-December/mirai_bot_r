use super::{
    bot_trait::{BotAction, EventHandler},
    custom_impl::{chat_listen, try_answer},
    group::GroupSender,
    message::{Message, MessageChain},
    my_bot::MyBot,
    summary_msg::accumulate_msg,
};
use crate::{setup::conf::APP_CONF, SENDER};
use async_trait::async_trait;
use rand::{thread_rng, Rng};

use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[async_trait]
impl EventHandler for MyBot {
    async fn handle_group_event(&'static self, message_chain: &MessageChain, sender: &GroupSender) {
        // let m = MessageChain::new()
        //     .build_target("902907141")
        //     .build_text("测试123");
        // SENDER.clone().get().unwrap().send(m).await.unwrap();

        thread::sleep(Duration::from_secs(1));
        let temp_chain = message_chain.get_message_chain();
        let mut message_chain: Vec<Message> = Vec::new();
        for ele in temp_chain {
            if ele._type.eq_ignore_ascii_case("Source") || ele._type.eq_ignore_ascii_case("Quote") {
                continue;
            }
            message_chain.push(ele.clone());
        }

        let message_chain = message_chain;

        // 执行逻辑
        if message_chain.len() == 0 {
            return;
        }

        let group_num = sender.get_group().id.to_string();
        let temp_chain = message_chain.clone();
        let temp_sender = sender.clone();
        tokio::task::spawn(async move { accumulate_msg(temp_chain, temp_sender).await });

        // if self.say_or_not_instruction(&message_chain, &group_num) {
        // return;
        // }

        // if self.summary_instruction(&message_chain, sender) {
        //     return;
        // }

        // chat_listen(&message_chain, &sender);
        // if !sender.get_group().id.to_string().eq(&APP_CONF.bot_group) {
        //     return;
        // }

        // if thread_rng().gen_range(0..10) < 6 {
        //     return;
        // }

        // try_answer(
        //     &message_chain,
        //     self,
        //     sender.get_group().id.to_string().as_str(),
        // );
    }

    async fn handle_nudge_event(&'static self, from_id: &String, target: &String, subject: &Value) {
        println!("事件发生!");
        if target.eq(&self.qq) {
            // if thread_rng().gen_range(0..10) > 6 {
            //     return;
            // }
            let msg = MessageChain::new()
                .build_at(String::from(from_id))
                .build_text("别戳我！") // https://api.vvhan.com/api/acgimg
                .build_img(String::from("https://t.mwm.moe/ycy"));
            self.send_group_nudge(subject["id"].to_string(), from_id.clone());
            // self.send_group_msg(&subject["id"].to_string(), &msg);
        }
    }
}
