use super::{
    bot_trait::{BotAction, EventHandler, GroupAdmin},
    group::GroupSender,
    message::{Message, MessageChain},
    my_bot::MyBot,
};
use crate::{
    api::{aitaffy::aitaffy, wx_chat::AI},
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
                let answer = (sender.get_id(), message_chain.to_vec());
                println!("ask:{:#?}\n answer:{:#?}", ask, answer);
                for ele in ask.1 {
                    match ele._type.as_str() {
                        "Plain" => set_ask_answer(&ele.text.unwrap(), &ask.0, &answer.0, &answer.1),
                        "Image" => {
                            set_ask_answer(&ele.image_id.unwrap(), &ask.0, &answer.0, &answer.1)
                        }
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
            "Plain" => match get_nearest_answer(ele.text.as_ref().unwrap()) {
                Some(answer) => {
                    println!("搜到答案，尝试回复！");
                    bot.send_group_msg(group_num, &MessageChain::from(answer))
                }
                None => println!("未找到Plain"),
            },
            "Image" => match get_nearest_answer(ele.image_id.as_ref().unwrap()) {
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
        // println!(
        //     "tidy message_chain:\n{:#?}",
        //     serde_json::to_string(&message_chain).unwrap()
        // ); // 去掉了Source

        match message_chain.len() {
            0 => (),
            2 => {
                // 如果是 at 机器人的消息
                if message_chain[0]._type.eq("At")
                    && message_chain[1]._type.eq("Plain")
                    && message_chain[0].target.unwrap().to_string().eq(&self.qq)
                {
                    // 匹配到指令
                    if message_chain[1]
                        .text
                        .as_ref()
                        .unwrap()
                        .contains("admin add")
                    {
                        let msg = message_chain[1].text.as_ref().unwrap();

                        let reg = Regex::new(r"admin add ([0-9]+)").unwrap();
                        println!("{:#?}", reg);
                        for (_, [qq]) in reg.captures_iter(&msg).map(|c| c.extract()) {
                            let res = self.member_admin(&APP_CONF.bot_group, qq, true);

                            if res.is_empty() {
                                println!("已添加 {} 为管理员~", qq);
                                let msg = MessageChain::new()
                                    .build_text(&format!("已添加 {} 为管理员~", qq));
                                self.send_group_msg(&APP_CONF.bot_group, &msg);
                            } else {
                                let msg = MessageChain::new()
                                    .build_text(&format!("添加失败, 失败原因: {}", res));

                                self.send_group_msg(&APP_CONF.bot_group, &msg);
                            }
                        }
                        return;
                    }

                    if message_chain[1]
                        .text
                        .as_ref()
                        .unwrap()
                        .contains("#poweroff")
                    {
                        let msg = MessageChain::new().build_text("已关机");

                        self.send_group_msg(&sender.get_group().id.to_string(), &msg);
                        exit(0);
                    }

                    if message_chain[1].text.as_ref().unwrap().contains("mute") {
                        let msg = MessageChain::new().build_text("小A 已沉默");

                        self.send_group_msg(&sender.get_group().id.to_string(), &msg);
                        self.mute();
                        return;
                    }

                    if message_chain[1].text.as_ref().unwrap().contains("active") {
                        self.active();
                        let msg = MessageChain::new().build_text("小A 开始活跃了！");
                        self.send_group_msg(&sender.get_group().id.to_string(), &msg);

                        return;
                    }

                    // 不是指令，且 at bot 则 AI 回复
                    let ans = self
                        .process_text(message_chain[1].text.as_ref().unwrap().as_str())
                        .replace("\\n", "\n")
                        .replace("\\", "");
                    let ans = MessageChain::new()
                        .build_at(sender.get_id())
                        .build_text(&ans);
                    // let mut tf = aitaffy(&ans);
                    // let mut voice = MessageChain::new();
                    // tf.iter_mut().for_each(|add|{
                    //     voice.build_voice(add);
                    // });

                    self.send_group_msg(sender.get_group().id.to_string().as_ref(), &ans);
                    // self.send_group_msg(sender.get_group().id.to_string().as_str(), &ans);
                    return;
                }

                // 一般的消息，偷听
                println!("{:#?}", message_chain);
                chat_listen(&message_chain, sender);
                try_answer(
                    &message_chain,
                    self,
                    sender.get_group().id.to_string().as_str(),
                );
            }

            _ => {
                chat_listen(&message_chain, sender);
                try_answer(
                    &message_chain,
                    self,
                    sender.get_group().id.to_string().as_str(),
                );
            }
        }

        // if !message_chain[1]._type.eq_ignore_ascii_case("Plain")
        //     || !message_chain[1].text.as_ref().unwrap().eq("测试3")
        // {
        //     if message_chain.len() == 2 {
        //         return;
        //     }

        //     if message_chain[1]._type.eq("At")
        //         && message_chain[1].target.unwrap().to_string().eq(&self.qq)
        //         && message_chain[2]._type.eq("Plain")
        //     {
        //         let ans = self.process_text(message_chain[2].text.as_ref().unwrap().as_str());
        //         let ans = MessageChain::new()
        //             .build_at(sender.get_id())
        //             .build_text(ans);
        //         self.send_group_msg(sender.get_group().id.to_string().as_str(), &ans);
        //     }

        //     return;
        // }

        // let msg = MessageChain::new()
        //     .build_text(String::from("你好！"))
        //     .build_text(String::from("晚上好！"))
        //     .build_img(String::from(
        //         "https://i0.hdslb.com/bfs/album/67fc4e6b417d9c68ef98ba71d5e79505bbad97a1.png",
        //     ))
        //     .build_at(String::from(sender.get_id()));
        // self.send_group_msg(&sender.get_group().id.to_string(), &msg);
        // self.send_group_nudge(sender.get_group().id.to_string(), sender.get_id())
    }

    fn handle_nudge_event(&self, from_id: &String, target: &String, subject: &Value) {
        println!("事件发生!");
        if target.eq(&self.qq) {
            // if thread_rng().gen_range(0..10) > 6 {
            //     return;
            // }
            let msg = MessageChain::new()
                .build_at(String::from(from_id))
                .build_text("别戳我！")
                .build_img(String::from("https://api.vvhan.com/api/acgimg"));
            self.send_group_nudge(subject["id"].to_string(), from_id.clone());
            self.send_group_msg(&subject["id"].to_string(), &msg);
        }
    }
}
