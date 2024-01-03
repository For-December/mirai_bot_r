use super::{
    bot_trait::{BotAction, EventHandler},
    custom_impl::try_answer,
    group::GroupSender,
    message::{Message, MessageChain},
    my_bot::MyBot,
    summary_msg::accumulate_msg,
};
use crate::{api::baidu_ocr::get_ocr_text, setup::conf::APP_CONF, SENDER};
use async_trait::async_trait;
// use rand::{thread_rng, Rng};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread::sleep,
    time::Duration,
};

lazy_static! {
    static ref IS_MUTE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref BAN_MAP: Arc<RwLock<HashMap<String, bool>>> = Arc::new(RwLock::new(HashMap::new()));
}

#[async_trait]
impl EventHandler for MyBot {
    async fn handle_group_event(&'static self, message_chain: MessageChain, sender: GroupSender) {
        // let m = MessageChain::new()
        //     .build_target("902907141")
        //     .build_text("测试123");
        // SENDER.clone().get().unwrap().send(m).await.unwrap();

        // thread::sleep(Duration::from_secs(1));
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

        // 艾特指令
        if message_chain[0]._type.eq("At")
            && message_chain[1]._type.eq("Plain")
            && message_chain[0].target.unwrap().to_string().eq(&self.qq)
        {
            match message_chain.len() {
                2 => {
                    if message_chain[1].text.as_ref().unwrap().contains("active") {
                        let msg = MessageChain::new()
                            .build_target(&group_num)
                            .build_text("小A 开始活跃了！");
                        SENDER.clone().get().unwrap().send(msg).await.unwrap();
                        let is_mute = Arc::clone(&IS_MUTE);
                        is_mute.store(false, Ordering::Release);
                        return;
                    }

                    if message_chain[1].text.as_ref().unwrap().contains("#recall") {
                        let is_bannd: bool;
                        {
                            // 如果用户被ban则直接结束
                            let ban_rw = Arc::clone(&BAN_MAP); // 引用计数器++
                            let ban_r = ban_rw.read().unwrap();

                            is_bannd = match ban_r.get(&sender.get_id()) {
                                Some(is_banned) => is_banned.clone(),
                                None => false,
                            };
                        } // 读锁在这里释放

                        if is_bannd {
                            let msg = MessageChain::new()
                                .build_target(&group_num.clone())
                                .build_text("别急，你的撤回还在冷却中...");
                            SENDER.clone().get().unwrap().send(msg).await.unwrap();
                            return;
                        }

                        {
                            // 设置ban状态
                            let ban_rw = Arc::clone(&BAN_MAP); // 引用计数器++
                            let mut ban_w = ban_rw.write().unwrap();
                            ban_w.insert(sender.get_id(), true);
                        } // 写锁在这里释放

                        tokio::task::spawn(async move {
                            // 一分钟后设为false
                            sleep(Duration::from_secs(30));
                            {
                                let ban_rw = Arc::clone(&BAN_MAP);
                                let mut ban_w = ban_rw.write().unwrap();
                                ban_w.insert(sender.get_id(), false);
                            }
                        });

                        self.recall_last_group_msg(group_num).await;
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
                        let res = SENDER
                            .clone()
                            .get()
                            .unwrap()
                            .send(msg)
                            .await
                            .is_ok_and(|_| {
                                let is_mute = Arc::clone(&IS_MUTE);
                                is_mute.store(true, Ordering::Release);
                                true
                            });
                        if res {
                            println!("成功沉默小A！");
                        }
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
                            .build_text("111，真关吗哥? ");
                        let res = SENDER
                            .clone()
                            .get()
                            .unwrap()
                            .send(ans)
                            .await
                            .is_ok_and(|_| {
                                tokio::task::spawn(async {
                                    sleep(Duration::from_secs(1));
                                    println!("已关机");
                                    // std::process::exit(0);
                                });
                                true
                            });
                        if !res {
                            println!("关机消息发送失败！！");
                            std::process::exit(0);
                            // return;
                        }
                        return;
                    }

                    if message_chain[1].text.as_ref().unwrap().contains("summary") {
                        tokio::task::spawn(Self::summary_instruction(
                            group_num.clone(),
                            sender.clone(),
                        ));
                        return;
                    }
                    if message_chain[1].text.as_ref().unwrap().contains("animes") {
                        tokio::task::spawn(Self::bilibili_instruction(sender.clone()));
                        return;
                    }
                    if message_chain[1]
                        .text
                        .as_ref()
                        .unwrap()
                        .contains("recommendations")
                    {
                        tokio::task::spawn(Self::bilibili_recommendations(sender.clone()));
                        return;
                    }
                    if message_chain[1].text.as_ref().unwrap().contains("magnet:?") {
                        let ans = MessageChain::new()
                            .build_target(&group_num)
                            .build_text("磁力链？");
                        SENDER.clone().get().unwrap().send(ans).await.unwrap();
                        //     let index = message_chain[1]
                        //         .text
                        //         .as_ref()
                        //         .unwrap()
                        //         .find("magnet:?")
                        //         .unwrap_or_default();
                        //     let magic_str = &message_chain[1].text.as_ref().unwrap()[index..];
                        //     tokio::task::spawn(Self::magic_instruction(
                        //         magic_str.to_string(),
                        //         sender.clone(),
                        //     ));
                        return;
                    }

                    if message_chain[1].text.as_ref().unwrap().contains("screen") {
                        let msg = message_chain[1].text.as_ref().unwrap();
                        let index = msg.find("http").unwrap();
                        let url = msg[index..].to_string();
                        println!("{}", url);
                        tokio::spawn(Self::get_screenshot(url, sender.clone()));
                        return;
                    }

                    tokio::task::spawn(Self::ai_chat(message_chain.clone(), sender.clone()));
                    return;
                }
                3 => {
                    if message_chain[1].text.as_ref().unwrap().contains("get_text")
                        && message_chain[2]._type.eq("Image")
                    {
                        tokio::task::spawn(async move {
                            let silce = get_ocr_text(message_chain[2].url.as_ref().unwrap()).await;
                            let text = format!("{:?}", silce);
                            let msg = MessageChain::new()
                                .build_target(&group_num)
                                .build_at(sender.get_id())
                                .build_text(&text);
                            SENDER.clone().get().unwrap().send(msg).await.unwrap();
                        });
                        return;
                    }

                    if message_chain[1].text.as_ref().unwrap().contains("comment")
                        && message_chain[2]._type.eq("Image")
                    {
                        let silce = get_ocr_text(message_chain[2].url.as_ref().unwrap()).await;
                        let text = format!("{:?}", silce);
                        let msg = MessageChain::new()
                            .build_target(&group_num)
                            .build_at(sender.get_id())
                            .build_text(&text);
                        SENDER.clone().get().unwrap().send(msg).await.unwrap();
                        return;
                    }
                }
                _ => {}
            }
        }

        // 用于偷听的记录
        tokio::task::spawn(Self::chat_listen(message_chain.clone(), sender.clone()));

        let is_mute = Arc::clone(&IS_MUTE);
        if is_mute.load(Ordering::Acquire) {
            return;
        }

        if thread_rng().gen_range(0..10) < 6 || !group_num.eq(APP_CONF.bot_group.as_str()) {
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
                .build_text("别戳我！"); // https://api.vvhan.com/api/acgimg
                                         // .build_img(String::from("http://127.0.0.1:9989/proxy")); // https://api.anosu.top/img/

            //https://t.mwm.moe/ycy
            SENDER.clone().get().unwrap().send(msg).await.unwrap();
            // self.send_group_msg(&subject["id"].to_string(), &msg);
            self.send_group_nudge(subject["id"].to_string(), from_id.clone())
                .await;
        }
    }
}
