// use std::collections::HashMap;
use mirai_bot::verify_and_bind;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    verify_and_bind()?;

    Ok(())
}
