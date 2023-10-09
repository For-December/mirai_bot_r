use super::{
    bot_trait::{BotAction, EventHandler, GroupAdmin},
    group::GroupSender,
    message::{Message, MessageChain},
    my_bot::MyBot,
};
use crate::{api::wx_chat::AI, setup::conf::APP_CONF};
use rand::{thread_rng, Rng};
use regex::Regex;
use serde_json::Value;
use std::{thread, time::Duration};

impl EventHandler for MyBot {
    fn handle_group_event(&self, message_chain: &MessageChain, sender: &GroupSender) {
        thread::sleep(Duration::from_secs(2));
        let message_chain = message_chain.get_message_chain();
        let mut store_chain: Vec<&Message> = Vec::new();
        for ele in message_chain {
            if ele._type.eq_ignore_ascii_case("Source") {
                continue;
            }
            store_chain.push(ele);
        }
        println!("{:#?}", serde_json::to_string(&store_chain).unwrap());
        match message_chain.len() {
            3 => {
                // 如果是 at 机器人的消息
                if message_chain[1]._type.eq("At")
                    && message_chain[2]._type.eq("Plain")
                    && message_chain[1].target.unwrap().to_string().eq(&self.qq)
                {
                    // 匹配到指令
                    if message_chain[2]
                        .text
                        .as_ref()
                        .unwrap()
                        .contains("admin add")
                    {
                        let msg = message_chain[2].text.as_ref().unwrap();

                        let reg = Regex::new(r"admin add ([0-9]+)").unwrap();
                        println!("{:#?}", reg);
                        for (_, [qq]) in reg.captures_iter(&msg).map(|c| c.extract()) {
                            let res = self.member_admin(&APP_CONF.bot_group, qq, true);

                            if res.is_empty() {
                                println!("已添加 {} 为管理员~", qq);
                                let msg = MessageChain::new()
                                    .build_text(format!("已添加 {} 为管理员~", qq));
                                self.send_group_msg(&APP_CONF.bot_group, &msg);
                            } else {
                                let msg = MessageChain::new()
                                    .build_text(format!("添加失败, 失败原因: {}", res));
                                self.send_group_msg(&APP_CONF.bot_group, &msg);
                            }
                        }
                        return;
                    }

                    // 不是指令，且 at bot 则 AI 回复
                    let ans = self.process_text(message_chain[2].text.as_ref().unwrap().as_str());
                    let ans = MessageChain::new()
                        .build_at(sender.get_id())
                        .build_text(ans);
                    self.send_group_msg(sender.get_group().id.to_string().as_str(), &ans);
                    return;
                }

                // 一般的消息，偷听
                println!("{:#?}", message_chain);
            }

            _ => return,
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
        if target.eq("3090807650") {
            if thread_rng().gen_range(0..10) < 6 {
                return;
            }
            let msg = MessageChain::new()
                .build_at(String::from(from_id))
                .build_text(String::from("别戳我！"))
                .build_img(String::from("https://api.vvhan.com/api/acgimg"));
            self.send_group_msg(&subject["id"].to_string(), &msg);
        }
    }
}
