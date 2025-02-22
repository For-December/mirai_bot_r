use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::Duration;
mod api;
mod bot;
mod database;
mod setup;
use bot::event::*;
use bot::my_bot::*;
use lazy_static::lazy_static;
use log::info;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::bot::bot_trait::BotAction;
use crate::bot::bot_trait::EventHandler;
// use crate::bot::message::Message;
use crate::bot::message::MessageChain;

lazy_static! {
    pub static ref MY_BOT: OnceLock<MyBot> = OnceLock::new();
    pub static ref MUTE_MAP: Arc<RwLock<HashMap<bool,String>>> = Arc::new(RwLock::new(HashMap::new()));
    // pub static ref IS_MUTE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref SENDER: OnceLock<Sender<MessageChain>> = OnceLock::new();
    pub static ref LAST_MSG: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

// impl AI for MyBot {}
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    //init
    MY_BOT.set(MyBot::build().await?).unwrap();
    println!("{:#?}", MY_BOT.get().unwrap());
    let (sender, mut receiver): (Sender<MessageChain>, Receiver<MessageChain>) = mpsc::channel(32);
    SENDER.set(sender).unwrap();

    tokio::spawn(async move {
        while let Some(message_chain) = receiver.recv().await {
            // println!("{:#?}", message_chain);
            match MY_BOT.get().unwrap().send_group_msg(&message_chain).await {
                Ok(message_id) => {
                    match message_chain._type {
                        Some(_type) => {
                            info!("不记录该消息！：{}", _type)
                        }
                        None => LAST_MSG.clone().lock().unwrap().push(message_id), // 添加上一条消息id
                    }
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    });

    loop {
        std::thread::sleep(Duration::from_secs(5));
        let events = MY_BOT.get().unwrap().get_events(10).await.unwrap();
        if events.is_none() {
            continue;
        }
        let events = events.unwrap();

        // 开新线程处理任务
        events.into_iter().for_each(|event| match event {
            Event::GroupEvent((message_chain, sender)) => {
                tokio::task::spawn(async move {
                    MY_BOT
                        .get()
                        .unwrap()
                        .handle_group_event(message_chain, sender)
                        .await;
                });
            }
            Event::NudgeEvent((from_id, target, subject)) => {
                tokio::task::spawn(async move {
                    MY_BOT
                        .get()
                        .unwrap()
                        .handle_nudge_event(&from_id, &target, &subject)
                        .await;
                });
            }
        });
    }

    // Ok(())
}
