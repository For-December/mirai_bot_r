use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Duration;
mod api;
mod bot;
mod database;
mod setup;
use async_once::AsyncOnce;
use bot::event::*;
use bot::my_bot::*;
use lazy_static::lazy_static;

use crate::bot::bot_trait::EventHandler;

lazy_static! {
    static ref MY_BOT: OnceLock<MyBot> = OnceLock::new();
    static ref IS_MUTE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

// impl AI for MyBot {}
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    MY_BOT.set(MyBot::build().await?).unwrap();
    println!("{:#?}", MY_BOT.get().unwrap());

    loop {
        std::thread::sleep(Duration::from_secs(5));
        let events = MY_BOT.get().unwrap().get_events(5).await.unwrap();
        if events.is_none() {
            continue;
        }
        let events = events.unwrap();
        events.iter().for_each(|event| match event {
            Event::GroupEvent((message_chain, sender)) => MY_BOT
                .get()
                .unwrap()
                .handle_group_event(message_chain, sender),
            Event::NudgeEvent((from_id, target, subject)) => MY_BOT
                .get()
                .unwrap()
                .handle_nudge_event(from_id, target, subject),
        });
    }

    Ok(())
}
