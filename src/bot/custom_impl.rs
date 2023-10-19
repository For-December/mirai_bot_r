use std::process::exit;

use rand::{thread_rng, Rng};
use regex::Regex;

use crate::{
    api::wx_chat::AI,
    bot::{
        bot_trait::{BotAction, GroupAdmin},
        message::MessageChain,
        summary_msg::summary,
    },
    database::mysql::{get_nearest_answer, set_ask_answer},
    setup::conf::APP_CONF,
};

use super::{group::GroupSender, message::Message, my_bot::MyBot};

impl MyBot {
    pub fn say_or_not_instruction(
        &mut self,
        message_chain: &Vec<Message>,
        group_num: &str,
    ) -> bool {
        if message_chain.len() != 2 {
            return false; // 表示该指令未运行
        }

        if message_chain[0]._type.eq("At")
            && message_chain[1]._type.eq("Plain")
            && message_chain[0].target.unwrap().to_string().eq(&self.qq)
        {
            if message_chain[1].text.as_ref().unwrap().contains("#help") {
                println!(
                    "{} {}",
                    message_chain[1].text.as_ref().unwrap(),
                    message_chain[1].text.as_ref().unwrap().eq(" #help"),
                );
                let msg = MessageChain::new().build_text(
                    r#"小A当前的指令如下:
#help --帮助
#mute --让小A沉默
#active --解除沉默
#bilibili --随机获取b站视频（待完成）
#post --在聊天室发帖（待完成）
                "#,
                );
                self.send_group_msg(&APP_CONF.bot_group, &msg);
                return true;
            }
            // 匹配到指令
            if message_chain[1]
                .text
                .as_ref()
                .unwrap()
                .contains("admin add")
            {
                let msg = message_chain[1].text.as_ref().unwrap();

                let reg = Regex::new(r"admin add ([0-9]+)").unwrap();
                // println!("{:#?}", reg);
                for (_, [qq]) in reg.captures_iter(&msg).map(|c| c.extract()) {
                    let res = self.member_admin(&APP_CONF.bot_group, qq, true);

                    if res.is_empty() {
                        println!("已添加 {} 为管理员~", qq);
                        let msg =
                            MessageChain::new().build_text(&format!("已添加 {} 为管理员~", qq));
                        self.send_group_msg(&APP_CONF.bot_group, &msg);
                    } else {
                        let msg =
                            MessageChain::new().build_text(&format!("添加失败, 失败原因: {}", res));

                        self.send_group_msg(&APP_CONF.bot_group, &msg);
                    }
                }
                return true;
            }

            if message_chain[1]
                .text
                .as_ref()
                .unwrap()
                .contains("#poweroff")
            {
                let msg = MessageChain::new().build_text("已关机");
                self.active();
                self.send_group_msg(group_num, &msg);
                exit(0);
            }

            if message_chain[1].text.as_ref().unwrap().contains("mute") {
                let msg = MessageChain::new().build_text("小A 已沉默");

                self.send_group_msg(group_num, &msg);
                self.mute();
                return true;
            }

            if message_chain[1].text.as_ref().unwrap().contains("active") {
                self.active();
                let msg = MessageChain::new().build_text("小A 开始活跃了！");
                self.send_group_msg(group_num, &msg);

                return true;
            }
        }
        return false;
    }

    pub fn summary_instruction(&self, message_chain: &Vec<Message>, sender: &GroupSender) -> bool {
        if message_chain.len() != 2 {
            return false; // 表示该指令未运行
        }

        if message_chain[0]._type.eq("At")
            && message_chain[1]._type.eq("Plain")
            && message_chain[0].target.unwrap().to_string().eq(&self.qq)
            && message_chain[1].text.as_ref().unwrap().contains("summary")
        {
            let mut msg = String::from(
                "下面是用户对话，格式为`昵称: 说话内容`，请帮我提取和总结其中的关键信息：\n",
            );
            msg += &summary(sender.get_group().id.to_string().as_str());
            println!("#######################\n{}\n#####################", msg);

            let ans = self
                .process_text(&msg)
                .replace("\\n", "\n")
                .replace("\\", "");
            let ans = MessageChain::new()
                .build_at(sender.get_id())
                .build_text(&ans);

            self.send_group_msg(sender.get_group().id.to_string().as_ref(), &ans);
            return true;
        }
        return false;
    }

    pub fn ai_chat(&self, message_chain: &Vec<Message>, sender: &GroupSender) -> bool {
        if message_chain.len() != 2 {
            return false; // 表示该指令未运行
        }

        if !message_chain[0]._type.eq("At")
            || !message_chain[1]._type.eq("Plain")
            || !message_chain[0].target.unwrap().to_string().eq(&self.qq)
        {
            return false;
        }
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
        return true;
    }
}
static mut GLOBAL_MSG: Vec<(String, Vec<Message>)> = Vec::new();

pub fn chat_listen(message_chain: &Vec<Message>, sender: &GroupSender) {
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
                // println!("ask:{:#?}\n answer:{:#?}", ask, answer);
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

pub fn try_answer(ask: &Vec<Message>, bot: &MyBot, group_num: &str) {
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
