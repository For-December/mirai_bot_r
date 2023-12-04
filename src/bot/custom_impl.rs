use std::{
    collections::HashMap,
    process::exit,
    sync::{Arc, Mutex, RwLock},
    thread::sleep,
    time::Duration,
};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use serde_json::Value;
use std::process;

use crate::{
    api::{
        bilibili::{get_bv_info, get_info, get_latest_anime, get_rcmd, get_video_summary},
        gpt_chat::AI,
        magic::get_preview,
    },
    bot::{message::MessageChain, summary_msg::summary},
    database::mysql::{get_nearest_answer, set_ask_answer},
    setup::conf::APP_CONF,
    SENDER,
};

use super::{
    api_utils::{get_bytes, get_bytes_with_body},
    group::GroupSender,
    message::Message,
    my_bot::MyBot,
};
impl AI for MyBot {}
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

                let msg = if res.is_empty() {
                    println!("已添加 {} 为管理员~", qq);
                    MessageChain::new().build_text(&format!("已添加 {} 为管理员~", qq))
                } else {
                    MessageChain::new().build_text(&format!("添加失败, 失败原因: {}", res))
                };
                SENDER.clone().get().unwrap().send(msg).await.unwrap();
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
            sleep(Duration::from_secs(5));
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

    pub async fn magic_instruction(magic_str: String, sender: GroupSender) {
        let res = get_preview(&magic_str).await;
        let size = res.size as f64;
        let mut msg = MessageChain::new()
            .build_target(sender.get_group().id.to_string().as_str())
            .build_at(sender.get_id())
            .build_text(
                format!(
                    "\n详情如下：\n name: {}\n type: {}\n size: {:.2}GB\n",
                    res.name,
                    res._type,
                    size / 1024.0 / 1024.0 / 1024.0
                )
                .as_str(),
            )
            .build_text("预览图自己点开看罢\n");
        for ele in res.screenshots.unwrap().into_iter() {
            let url = ele["screenshot"]
                .to_string()
                .trim_matches('"')
                .trim_matches(' ')
                .to_string();
            let base64_url = get_bytes(&url).await.unwrap();
            msg.ref_build_img(base64_url);
            // msg.ref_build_img(url);
            // msg.ref_build_text(&url);
            break;
        }
        SENDER.clone().get().unwrap().send(msg).await.unwrap();
    }

    pub async fn get_screenshot(url: String, sender: GroupSender) -> bool {
        match get_bytes_with_body("http://localhost:9876/screen", url).await {
            Ok(base64) => {
                let msg = MessageChain::new()
                    .build_target(sender.get_group().id.to_string().as_str())
                    .build_at(sender.get_id())
                    .build_img(base64);
                SENDER.clone().get().unwrap().send(msg).await.unwrap();
                return true;
            }
            Err(err) => {
                let msg = MessageChain::new()
                    .build_target(sender.get_group().id.to_string().as_str())
                    .build_at(sender.get_id())
                    .build_text(&format!("出错了：{}", err));
                SENDER.clone().get().unwrap().send(msg).await.unwrap();
                return false;
            }
        }
    }
    pub async fn sniff_bilibili_video(msg: Message, sender: GroupSender) -> bool {
        let text = match msg._type.as_str() {
            "Plain" => msg.text.unwrap().to_string(),
            "App" => {
                let json = msg.content.unwrap();
                let json: Value = serde_json::from_str(&json).unwrap();
                println!("{:#?}", json);
                let url = json["meta"]["detail_1"]["qqdocurl"].to_string();
                println!("{}", url);
                url.trim_matches('"').to_string()
            }
            _ => {
                return false;
            }
        };
        match get_info(&text).await {
            Ok(info) => {
                tokio::task::spawn(async move {
                    {
                        let msg = MessageChain::new()
                            .build_target(sender.get_group().id.to_string().as_str())
                            .build_at(sender.get_id())
                            .build_text(
                                format!(
                                    "\n标题：{}\n作者: {}\n描述：{}\n链接：{}\n",
                                    info.title, info.owner_name, info.desc, info.url,
                                )
                                .as_str(),
                            )
                            .build_img(info.pic);
                        SENDER.clone().get().unwrap().send(msg).await.unwrap();
                    }

                    {
                        let msg = MessageChain::new()
                            .build_target(sender.get_group().id.to_string().as_str())
                            .build_at(sender.get_id())
                            .build_text(&format!(
                                "ai总结如下\n{}",
                                get_video_summary(info.url).await
                            ));
                        SENDER.clone().get().unwrap().send(msg).await.unwrap();
                    }
                });
                return true;
            }
            Err(_) => {
                // println!("未成功获取bv信息！\n{}", err);
                return false;
            }
        }
    }
    pub async fn sniff_magic_chain(msg: Message, sender: &GroupSender) -> bool {
        if !msg._type.eq("Plain") {
            return false;
        }
        if msg.text.as_ref().unwrap().contains("magnet") {
            let index = msg
                .text
                .as_ref()
                .unwrap()
                .find("magnet")
                .unwrap_or_default();
            let magic_str = &msg.text.as_ref().unwrap()[index..];
            tokio::task::spawn(Self::magic_instruction(
                magic_str.replace("：", ":").replace("？", "?").to_string(),
                sender.clone(),
            ));
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

    pub async fn bilibili_recommendations(sender: GroupSender) -> bool {
        let res = get_rcmd().await;
        let mut ans = MessageChain::new()
            .build_target(sender.get_group().id.to_string().as_str())
            .build_at(sender.get_id())
            .build_text("\n今日份哔哩哔哩视频推荐：\n");
        let mut count = 0;
        for ele in &res {
            count += 1;
            if count > 5 {
                break;
            }
            ans.ref_build_text((ele.2.to_string() + "\n").as_str());
            ans.ref_build_text((ele.0.to_string() + "\n").as_str());
            ans.ref_build_img(ele.1.to_string().trim_matches('\"').to_string() + "\n");
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

        let ans = Self::process_text(&sender.get_member_name(), &msg)
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
            &sender.get_member_name(),
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
                .build_at(sender.get_id())
                .build_text("我已经忘掉了之前的故事，让我们重新开始吧~");
            SENDER.clone().get().unwrap().send(ans).await.unwrap();
            return true;
        }
        match MyBot::cat_girl(
            &sender.get_member_name(),
            message_chain[1].text.as_ref().unwrap().as_str(),
        )
        .await
        {
            0 => {
                let ans = MessageChain::new()
                    .build_target(sender.get_group().id.to_string().as_str())
                    .build_at(sender.get_id())
                    .build_text("预设加载成功！");
                SENDER.clone().get().unwrap().send(ans).await.unwrap();
                return true;
            }
            1 => {
                let ans = MessageChain::new()
                    .build_target(sender.get_group().id.to_string().as_str())
                    .build_at(sender.get_id())
                    .build_text("没有这个预设~");
                SENDER.clone().get().unwrap().send(ans).await.unwrap();
                return false;
            }
            _ => {}
        }

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

    pub async fn chat_listen(message_chain: Vec<Message>, sender: GroupSender) {
        // 先处理消息
        let mut new_message_chain = Vec::<Message>::new();
        for ele in message_chain {
            let text_len = utf8_slice::len(&ele.text.clone().unwrap_or_default());
            // 有过长的消息（小作文），则概率记录
            if text_len > 120 {
                continue; // 小作文大于 120 直接不记录
            }

            if text_len > 0 && Self::sniff_magic_chain(ele.clone(), &sender).await {
                println!("磁力链");
                // 是磁力链
                continue;
            }

            if Self::sniff_bilibili_video(ele.clone(), sender.clone()).await {
                println!("bilibili");
                continue;
            }

            // [0-120) [51-120)
            // 不记录概率 51/120 ~ 119/120
            // 记录概率 69/120 ~ 1/120
            if text_len > 50 && thread_rng().gen_range(0..120) < text_len {
                continue;
            }

            // 所有imgId清空，ask不清空
            // ele.image_id = None;
            new_message_chain.push(ele);
        }

        // 如果没有ask，直接当作ask存进去，否则读出ask，ask和群号关联
        // 是查询当前群号对应的 Vec，当有一个ask的时候当前作为answer
        let ask = get_ask(&new_message_chain, &sender).await;

        // 读出了 ask，则当前新消息就是answer
        if let Some(ask) = ask {
            let temp_answer = { (sender.get_id(), new_message_chain.to_vec()) };
            let mut answer = { (sender.get_id(), Vec::new()) };
            for mut ele in temp_answer.1.into_iter() {
                // 所有imgId清空
                ele.image_id = None;
                answer.1.push(ele);
            }
            // println!("ask:{:#?}\n answer:{:#?}", ask, answer);
            for ele in ask.1 {
                match ele._type.as_str() {
                    "Plain" => {
                        let ask_text = ele.clone().text.unwrap();
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
                        let temp = ele.clone();
                        set_ask_answer(
                            // 如果是 base64，可能会出问题
                            ele.image_id
                                .unwrap_or_else(move || {
                                    println!("{:#?}", temp);
                                    process::exit(1);
                                })
                                .as_str(),
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
}
lazy_static! {
    // Arc<RwLock<HashMap<String, Arc<Mutex<Vec<Conversation>>>>>>
    static ref GLOBAL_MSG: Arc<RwLock<HashMap<i64,Mutex<Vec<(String, Vec<Message>)>>>>>
    =Arc::new(RwLock::new(HashMap::new()));
}

pub async fn get_ask(
    message_chain: &Vec<Message>,
    sender: &GroupSender,
) -> Option<(String, Vec<Message>)> {
    let global_msg_map_rw = Arc::clone(&GLOBAL_MSG); // 带读写锁的
    let global_msg_map_r = global_msg_map_rw.read().unwrap(); // 读
    let global_msg = global_msg_map_r
        .get(&sender.get_group().id)
        .unwrap_or_else(|| {
            // 不存在key则插入
            global_msg_map_rw
                .write()
                .unwrap()
                .insert(sender.get_group().id, Mutex::new(Vec::new()));
            global_msg_map_r.get(&sender.get_group().id).unwrap()
        });
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

pub async fn try_answer(ask: Vec<Message>, group_num: String) {
    if thread_rng().gen_range(0..10) < 6 || !group_num.eq(APP_CONF.bot_group.as_str()) {
        return;
    }
    for ele in ask {
        match ele._type.as_str() {
            // "Plain" | "Image" => (),
            "Plain" => {
                if ele.text.as_ref().unwrap().len() < 2 {
                    continue;
                }
                match get_nearest_answer(ele.text.as_ref().unwrap(), group_num.as_str()).await {
                    Some(answer) => {
                        println!("搜到答案，尝试回复！");
                        println!("ask_text = {}", ele.text.as_ref().unwrap());
                        let mut ans = Vec::new();
                        for ele in answer {
                            if ele._type.contains("At") {
                                // continue;
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
                                // continue;
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
