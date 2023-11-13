use super::{
    bot_trait::{BotAction, EventHandler},
    custom_impl::{chat_listen, try_answer},
    group::GroupSender,
    message::{Message, MessageChain},
    my_bot::MyBot,
    summary_msg::accumulate_msg,
};
use crate::{api::bilibili, setup::conf::APP_CONF, MY_BOT, SENDER};
use async_trait::async_trait;
use rand::{thread_rng, Rng};

use lazy_static::lazy_static;
use serde_json::Value;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, sleep},
    time::Duration,
};

lazy_static! {
    static ref IS_MUTE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

#[async_trait]
impl EventHandler for MyBot {
    async fn handle_group_event(&'static self, message_chain: MessageChain, sender: GroupSender) {
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

        // 用于总结的记录
        tokio::task::spawn(accumulate_msg(message_chain.clone(), sender.clone()));
        // 用于偷听的记录
        tokio::task::spawn(chat_listen(message_chain.clone(), sender.clone()));

        if message_chain.len() == 2
            && message_chain[0]._type.eq("At")
            && message_chain[1]._type.eq("Plain")
            && message_chain[0].target.unwrap().to_string().eq(&self.qq)
        {
            if message_chain[1].text.as_ref().unwrap().contains("active") {
                let msg = MessageChain::new()
                    .build_target(&group_num)
                    .build_text("小A 开始活跃了！");
                SENDER.clone().get().unwrap().send(msg).await.unwrap();
                let is_mute = Arc::clone(&IS_MUTE);
                is_mute.store(false, Ordering::Release);
                return;
            }

            let is_mute = Arc::clone(&IS_MUTE);
            if is_mute.load(Ordering::Acquire) {
                return;
            }

            if message_chain[1].text.as_ref().unwrap().contains("mute") {
                let msg = MessageChain::new()
                    .build_target(&group_num)
                    .build_text("小A 已沉默");
                SENDER.clone().get().unwrap().send(msg).await.unwrap();
                let is_mute = Arc::clone(&IS_MUTE);
                is_mute.store(true, Ordering::Release);
                return;
            }
            if message_chain[1]
                .text
                .as_ref()
                .unwrap()
                .contains("#poweroff")
            {
                let ans = MessageChain::new()
                    .build_target(&group_num)
                    .build_text("已关机");
                SENDER.clone().get().unwrap().send(ans).await.unwrap();

                sleep(Duration::from_secs(3));

                std::process::exit(0);
            }

            if message_chain[1].text.as_ref().unwrap().contains("summary") {
                tokio::task::spawn(Self::summary_instruction(group_num.clone(), sender.clone()));
                return;
            }
            if message_chain[1].text.as_ref().unwrap().contains("animes") {
                tokio::task::spawn(Self::bilibili_instruction(sender.clone()));
                return;
            }
            if message_chain[1].text.as_ref().unwrap().contains("magnet:?") {
                let index = message_chain[1]
                    .text
                    .as_ref()
                    .unwrap()
                    .find("magnet:?")
                    .unwrap_or_default();
                let magic_str = &message_chain[1].text.as_ref().unwrap()[index..];
                tokio::task::spawn(Self::magic_instruction(
                    magic_str.to_string(),
                    sender.clone(),
                ));
                return;
            }

            tokio::task::spawn(Self::ai_chat(message_chain.clone(), sender.clone()));
        }

        let is_mute = Arc::clone(&IS_MUTE);
        if is_mute.load(Ordering::Acquire) {
            return;
        }
        tokio::task::spawn(try_answer(
            message_chain.clone(),
            sender.get_group().id.to_string(),
        ));
        // if self.say_or_not_instruction(&message_chain, &group_num) {
        // return;
        // }

        // if !sender.get_group().id.to_string().eq(&APP_CONF.bot_group) {
        //     return;
        // }

        // if thread_rng().gen_range(0..10) < 6 {
        //     return;
        // }
    }

    async fn handle_nudge_event(&'static self, from_id: &String, target: &String, subject: &Value) {
        println!("事件发生!");
        if target.eq(&self.qq) {
            // if thread_rng().gen_range(0..10) > 6 {
            //     return;
            // }
            let msg = MessageChain::new()
                .build_target(subject["id"].to_string().as_str())
                .build_at(String::from(from_id))
                .build_text("别戳我！") // https://api.vvhan.com/api/acgimg
                .build_img(String::from("https://t.mwm.moe/ycy"));
            SENDER.clone().get().unwrap().send(msg).await.unwrap();
            // self.send_group_msg(&subject["id"].to_string(), &msg);
            self.send_group_nudge(subject["id"].to_string(), from_id.clone())
                .await;
        }
    }
}
