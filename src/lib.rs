use std::time::Duration;

use crate::api::web_tools::MessageChain;

mod api;
pub fn verify_and_bind() -> Result<(), Box<dyn std::error::Error>> {
    let my_bot = api::web_tools::MyBot::new()?;
    println!("{:?}", my_bot);
    loop {
        my_bot.get_events(5)?;
        std::thread::sleep(Duration::from_secs(5));
    }

    let msg = MessageChain::new()
        .build_text(String::from("你好！"))
        .build_text(String::from("晚上好！"))
        .build_img(String::from(
            "https://i0.hdslb.com/bfs/album/67fc4e6b417d9c68ef98ba71d5e79505bbad97a1.png",
        ))
        .build_at(String::from(""));
    my_bot.send_group_msg(&String::from(""), &msg)?;

    Ok(())
}
