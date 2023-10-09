use std::time::Duration;
mod api;
mod bot;
mod setup;
use bot::event::*;
use bot::my_bot::*;

use crate::bot::bot_trait::EventHandler;

// impl AI for MyBot {}
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let my_bot = MyBot::new()?;
    println!("{:?}", my_bot);
    loop {
        std::thread::sleep(Duration::from_secs(5));
        let events = my_bot.get_events(5)?;
        if events.is_none() {
            continue;
        }
        let events = events.unwrap();
        events.iter().for_each(|event| match event {
            Event::GroupEvent((message_chain, sender)) => {
                my_bot.handle_group_event(message_chain, sender)
            }
            Event::NudgeEvent((from_id, target, subject)) => {
                my_bot.handle_nudge_event(from_id, target, subject)
            }
        });
    }

    Ok(())
}
