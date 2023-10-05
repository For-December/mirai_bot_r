use std::time::Duration;

use api::web_tools::GroupSender;

use crate::api::web_tools::{Event, MessageChain};

mod api;
pub fn verify_and_bind() -> Result<(), Box<dyn std::error::Error>> {
    let my_bot = api::web_tools::MyBot::new()?;
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
        });
    }

    let msg = MessageChain::new()
        .build_text(String::from("你好！"))
        .build_text(String::from("晚上好！"))
        .build_img(String::from(
            "https://i0.hdslb.com/bfs/album/67fc4e6b417d9c68ef98ba71d5e79505bbad97a1.png",
        ))
        .build_at(String::from("1921567337"));
    // my_bot.send_group_msg(&String::from("***REMOVED***"), &msg)?;

    Ok(())
}
