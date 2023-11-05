use std::{
    f32::consts::E,
    process::exit,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;

use crate::{
    api::{aitaffy::aitaffy, bilibili::get_latest_anime, wx_chat::AI},
    bot::{message::MessageChain, summary_msg::summary},
    database::mysql::{get_nearest_answer, set_ask_answer},
    setup::conf::APP_CONF,
    SENDER,
};

use super::{group::GroupSender, message::Message, my_bot::MyBot};

impl MyBot {
    pub async fn control_instrument(message_chain: &Vec<Message>, group_num: &str) -> bool {
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

            SENDER.clone().get().unwrap().send(msg).await.unwrap();
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
                let res = String::new();
                // let res = self.member_admin(&APP_CONF.bot_group, qq, true);

                if res.is_empty() {
                    println!("已添加 {} 为管理员~", qq);
                    let msg = MessageChain::new().build_text(&format!("已添加 {} 为管理员~", qq));
                } else {
                    let msg =
                        MessageChain::new().build_text(&format!("添加失败, 失败原因: {}", res));
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
            let ans = MessageChain::new()
                .build_target(group_num)
                .build_text("已关机");

            SENDER.clone().get().unwrap().send(ans).await.unwrap();

            exit(0);
        }

        if message_chain[1].text.as_ref().unwrap().contains("mute") {
            let msg = MessageChain::new().build_text("小A 已沉默");

            SENDER.clone().get().unwrap().send(msg).await.unwrap();

            return true;
        }

        if message_chain[1].text.as_ref().unwrap().contains("active") {
            let msg = MessageChain::new().build_text("小A 开始活跃了！");
            SENDER.clone().get().unwrap().send(msg).await.unwrap();

            return true;
        }

        return false;
    }

    pub async fn bilibili_instruction(sender: GroupSender) -> bool {
        let res = get_latest_anime().await;
        let mut ans = MessageChain::new()
            .build_target(sender.get_group().id.to_string().as_str())
            .build_at(sender.get_id())
            .build_text("\nb站最近更新的番剧来啦：\n");
        let mut count = 0;
        for ele in &res {
            count += 1;
            if count > 5 {
                break;
            }
            ans.ref_build_text((ele.0.to_string() + "\n => ").as_str());
            ans.ref_build_text((ele.1.to_string() + "\n").as_str());
            ans.ref_build_img(ele.2.to_string().trim_matches('\"').to_string());
        }
        SENDER.clone().get().unwrap().send(ans).await.unwrap();

        return true;
    }

    pub async fn summary_instruction(group_num: String, sender: GroupSender) -> bool {
        let mut msg = String::from(
            "下面是用户对话，格式为`昵称: 说话内容`，请帮我提取和总结其中的关键信息：\n",
        );
        msg += &summary(sender.get_group().id.to_string().as_str());
        println!("#######################\n{}\n#####################", msg);

        let ans = Self::process_text(&sender.get_id(), &msg)
            .await
            .replace("\\n", "\n")
            .replace("\\", "");
        let ans = MessageChain::new()
            .build_target(&group_num)
            .build_at(sender.get_id())
            .build_text(&ans);
        SENDER.clone().get().unwrap().send(ans).await.unwrap();
        return true;
    }

    pub async fn ai_chat(message_chain: Vec<Message>, sender: GroupSender) -> bool {
        if MyBot::debug(
            &sender.get_id(),
            message_chain[1].text.as_ref().unwrap().as_str(),
        )
        .await
        {
            return true;
        }
        if MyBot::forget(
            &sender.get_member_name(),
            message_chain[1].text.as_ref().unwrap().as_str(),
        )
        .await
        {
            let ans = MessageChain::new()
                .build_target(sender.get_group().id.to_string().as_str())
                .build_at(sender.get_member_name())
                .build_text("我已经忘掉了之前的故事，让我们重新开始吧~");
            SENDER.clone().get().unwrap().send(ans).await.unwrap();
            return true;
        }
        MyBot::cat_girl(
            &sender.get_member_name(),
            message_chain[1].text.as_ref().unwrap().as_str(),
        )
        .await;

        let ans = MyBot::process_text(
            &sender.get_member_name(),
            (sender.get_member_name() + message_chain[1].text.as_ref().unwrap().as_str()).as_str(),
        )
        .await
        .replace("\\n", "\n")
        .replace("\\", "");
        // let temp = ans.clone();
        let ans = MessageChain::new()
            .build_target(sender.get_group().id.to_string().as_str())
            .build_at(sender.get_id())
            .build_text(&ans);
        SENDER.clone().get().unwrap().send(ans).await.unwrap();

        // let voice = MessageChain::new()
        //     .build_target(sender.get_group().id.to_string().as_str())
        //     .build_voice(aitaffy(&temp).last().unwrap());
        // SENDER.clone().get().unwrap().send(voice).await.unwrap();
        // let mut tf = aitaffy(&ans);
        // let mut voice = MessageChain::new();
        // tf.iter_mut().for_each(|add|{
        //     voice.build_voice(add);
        // });
        return true;
    }
}
lazy_static! {
    static ref GLOBAL_MSG: Arc<Mutex<Vec<(String, Vec<Message>)>>> =
        Arc::new(Mutex::new(Vec::new()));
}

pub async fn get_ask(
    message_chain: &Vec<Message>,
    sender: &GroupSender,
) -> Option<(String, Vec<Message>)> {
    let global_msg = Arc::clone(&GLOBAL_MSG);
    let mut global_msg = global_msg.lock().unwrap();
    match global_msg.len() {
        0 => {
            global_msg.push((sender.get_id(), message_chain.to_vec()));
            None
        }
        1 => global_msg.pop(),
        _ => panic!("预期之外的消息数！"),
    }
}
pub async fn chat_listen(message_chain: Vec<Message>, sender: GroupSender) {
    let ask = get_ask(&message_chain, &sender).await;
    if let Some(ask) = ask {
        let answer = {
            let mut new_message_chain = Vec::<Message>::new();
            for mut ele in message_chain {
                let text_len = utf8_slice::len(&ele.text.clone().unwrap_or_default());
                // 有过长的消息（小作文），则概率记录
                if text_len > 120 {
                    continue; // 小作文大于 120 直接不记录
                }

                // [0-120) [51-120)
                // 不记录概率 51/120 ~ 119/120
                // 记录概率 69/120 ~ 1/120
                if text_len > 50 && thread_rng().gen_range(0..120) < text_len {
                    continue;
                }
                // 所有imgId清空
                ele.image_id = None;
                new_message_chain.push(ele);
            }
            (sender.get_id(), new_message_chain.to_vec())
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
                        &sender.get_group().id.to_string(),
                        &ask.0,
                        &answer.0,
                        &answer.1,
                    )
                    .await;
                }
                "Image" => {
                    set_ask_answer(
                        ele.image_id.unwrap().as_str(),
                        &sender.get_group().id.to_string(),
                        &ask.0,
                        &answer.0,
                        &answer.1,
                    )
                    .await;
                }
                _ => return,
            }
        }
    } else {
        return;
    }
    // match global_msg.len() {
    //     0 => global_msg.push((sender.get_id(), message_chain.to_vec())),
    //     1 => {
    //         let ask = global_msg.pop().unwrap();
    //
    //     }
    //     _ => panic!("预期之外的消息数！"),
    // }
}

pub async fn try_answer(ask: Vec<Message>, group_num: String) {
    if thread_rng().gen_range(0..10) < 6 || !group_num.eq(APP_CONF.bot_group.as_str()) {
        return;
    }
    for ele in ask {
        match ele._type.as_str() {
            // "Plain" | "Image" => (),
            "Plain" => {
                match get_nearest_answer(ele.text.as_ref().unwrap(), group_num.as_str()).await {
                    Some(answer) => {
                        println!("搜到答案，尝试回复！");
                        let mut ans = Vec::new();
                        for ele in answer {
                            if ele._type.contains("At") {
                                continue;
                            }
                            ans.push(ele);
                        }
                        SENDER
                            .clone()
                            .get()
                            .unwrap()
                            .send(MessageChain::from(Some(group_num.clone()), ans))
                            .await
                            .unwrap();
                    }
                    None => println!("未找到Plain"),
                }
            }
            "Image" => {
                match get_nearest_answer(ele.image_id.as_ref().unwrap(), group_num.as_str()).await {
                    Some(answer) => {
                        let mut ans = Vec::new();
                        for ele in answer {
                            if ele._type.contains("At") {
                                continue;
                            }
                            ans.push(ele);
                        }
                        println!("搜到答案，尝试回复！");
                        SENDER
                            .clone()
                            .get()
                            .unwrap()
                            .send(MessageChain::from(Some(group_num.clone()), ans))
                            .await
                            .unwrap();
                    }
                    None => println!("未找到Image"),
                }
            }
            _ => (),
        }
    }
}
