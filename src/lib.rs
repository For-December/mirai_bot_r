mod api;
pub fn verify_and_bind() -> Result<(), Box<dyn std::error::Error>> {
    let my_bot = api::web_tools::MyBot::new()?;
    println!("{:?}",my_bot);

    Ok(())
}
