// use std::collections::HashMap;
mod setup;
use mirai_bot::run;
use setup::logger::init_log;
// use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_log();

    run().await?;
    Ok(())
}
