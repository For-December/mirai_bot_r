use super::{
    bot_trait::{BotAction, EventHandler, GroupAdmin},
    group::GroupSender,
    message::{Message, MessageChain},
    my_bot::MyBot,
    summary_msg::accumulate_msg,
};
use crate::{
    api::{aitaffy::aitaffy, chatgpt::AI},
    bot::summary_msg::summary,
    database::mysql::{get_nearest_answer, set_ask_answer},
    setup::conf::APP_CONF,
};
use rand::{thread_rng, Rng};
use regex::Regex;
use serde_json::Value;
use std::{process::exit, thread, time::Duration};
static mut GLOBAL_MSG: Vec<(String, Vec<Message>)> = Vec::new();
fn chat_listen(message_chain: &Vec<Message>, sender: &GroupSender) {
    unsafe {
        match GLOBAL_MSG.len() {
            0 => GLOBAL_MSG.push((sender.get_id(), message_chain.to_vec())),
            1 => {
                let ask = GLOBAL_MSG.pop().unwrap();
                let answer = {
                    for ele in message_chain {
                        let text_len = utf8_slice::len(&ele.text.clone().unwrap_or_default());

                        // 有过长的消息（小作文），则概率记录
                        if text_len > 120 {
                            return; // 小作文大于 120 直接不记录
                        }

                        // [0-120) [51-120)
                        // 不记录概率 51/120 ~ 119/120
                        // 记录概率 69/120 ~ 1/120
                        if text_len > 50 && thread_rng().gen_range(0..120) < text_len {
                            return;
                        }
                    }
                    (sender.get_id(), message_chain.to_vec())
                };
                println!("ask:{:#?}\n answer:{:#?}", ask, answer);
                for ele in ask.1 {
                    match ele._type.as_str() {
                        "Plain" => {
                            let ask_text = ele.text.unwrap();
                            let mut ask_text = ask_text.as_str();
                            if utf8_slice::len(ask_text) > 255 {
                                ask_text = utf8_slice::slice(ask_text, 0, 255);
                            }
                            set_ask_answer(
                                ask_text,
                                sender.get_group().id.to_string().as_str(),
                                &ask.0,
                                &answer.0,
                                &answer.1,
                            )
                        }
                        "Image" => set_ask_answer(
                            &ele.image_id.unwrap(),
                            sender.get_group().id.to_string().as_str(),
                            &ask.0,
                            &answer.0,
                            &answer.1,
                        ),
                        _ => return,
                    }
                }
            }
            _ => panic!("预期之外的消息数！"),
        }
    }
}

fn try_answer(ask: &Vec<Message>, bot: &MyBot, group_num: &str) {
    // if thread_rng().gen_range(0..10) < 6 {
    // return;
    // }
    for ele in ask {
        match ele._type.as_str() {
            // "Plain" | "Image" => (),
            "Plain" => match get_nearest_answer(ele.text.as_ref().unwrap(), group_num) {
                Some(answer) => {
                    println!("搜到答案，尝试回复！");
                    bot.send_group_msg(group_num, &MessageChain::from(answer))
                }
                None => println!("未找到Plain"),
            },
            "Image" => match get_nearest_answer(ele.image_id.as_ref().unwrap(), group_num) {
                Some(answer) => {
                    println!("搜到答案，尝试回复！");
                    bot.send_group_msg(group_num, &MessageChain::from(answer))
                }
                None => println!("未找到Image"),
            },
            _ => (),
        }
    }
}

impl EventHandler for MyBot {
    fn handle_group_event(&mut self, message_chain: &MessageChain, sender: &GroupSender) {
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
        accumulate_msg(&message_chain, sender);

        if self.say_or_not_instruction(&message_chain, &group_num) {
            return;
        }
        if self.is_mute {
            return;
        }

        if self.summary_instruction(&message_chain, sender) {
            return;
        }

        self.ai_chat(&message_chain, &sender);

        chat_listen(&message_chain, &sender);
        if !sender.get_group().id.to_string().eq(&APP_CONF.bot_group) {
            return;
        }

        if thread_rng().gen_range(0..10) < 6 {
            return;
        }

        try_answer(
            &message_chain,
            self,
            sender.get_group().id.to_string().as_str(),
        );
    }

    fn handle_nudge_event(&self, from_id: &String, target: &String, subject: &Value) {
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
            self.send_group_msg(&subject["id"].to_string(), &msg);
        }
    }
}
