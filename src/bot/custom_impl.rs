use std::process::exit;

use regex::Regex;

use crate::{
    api::chatgpt::AI,
    bot::{
        bot_trait::{BotAction, GroupAdmin},
        message::MessageChain,
        summary_msg::summary,
    },
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
