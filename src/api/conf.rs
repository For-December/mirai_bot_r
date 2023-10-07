#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    pub base_url: String,
    pub verify_key: String,
    pub bot_qq: String,
}
