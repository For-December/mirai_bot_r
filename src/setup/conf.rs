use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub base_url: String,
    pub verify_key: String,
    pub bot_qq: String,
    pub bot_group: String,
    pub wx_api: WXApi,
    pub gpt_api: GPTApi,
}

#[derive(Debug, Deserialize)]
pub struct WXApi {
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Debug, Deserialize)]
pub struct GPTApi {
    pub api_key: String,
    pub end_point: String,
}

lazy_static! {
    pub static ref APP_CONF: AppConfig = init_conf("config.yaml").unwrap();
}

fn init_conf(file_path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut config_file = File::open(file_path)?;
    let mut config_yaml = String::new();

    // 读取配置文件内容
    config_file.read_to_string(&mut config_yaml)?;

    // 解析 YAML 配置文件
    let config: AppConfig = serde_yaml::from_str(&config_yaml)?;
    println!("{:?}", config);
    Ok(config)
}
