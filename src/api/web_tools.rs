use serde_json::Value;
use std::collections::HashMap;

pub const BASE_URL: &'static str = "http://127.0.0.1:8087";
pub const VERIFY_KEY: &'static str = "INITKEY2UnuZcms";
pub const BOT_QQ: &'static str = "***REMOVED***";

#[derive(Debug)]
pub struct MyBot {
    qq: String,
    session_key: String,
}

impl MyBot {
    pub fn new() -> Result<MyBot, Box<dyn std::error::Error>> {
        let session_key = get_verify()?;
        let qq = String::from(BOT_QQ);
        bind_release_verify(&session_key, BOT_QQ, false)?;
        Ok(MyBot { session_key, qq })
    }
}
// 实现 Drop trait
impl Drop for MyBot {
    fn drop(&mut self) {
        // 在这里执行资源的清理操作，例如关闭文件、释放内存等
        println!("Dropping MyBot...");
        bind_release_verify(&self.session_key, &self.qq, true).unwrap();

        // 在 drop 方法中执行异步清理操作
        // tokio::task::spawn_blocking(|| {
        // code async
        // });
        // let session_key = self.session_key.clone();
        // let res = tokio::spawn(async move {
        //     // 所有权转交给该线程，此时self可能已失效
        //     bind_release_verify(&session_key, BOT_QQ, true)
        //         .await
        //         .unwrap();
        //     println!("dropped!");
        // });
    }
}
fn blocking_post_msg(
    map: HashMap<&str, &str>,
    url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", url);
    let res = reqwest::blocking::Client::new()
        .post(url)
        .json(&map)
        .send()?
        .text()?;
    println!("{:#?}", res);
    Ok(res)
}

async fn post_msg(
    map: HashMap<&str, &str>,
    url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", url);
    let res = reqwest::Client::new()
        .post(url)
        .json(&map)
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", res);
    Ok(res)
}

pub fn get_verify() -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("verifyKey", VERIFY_KEY);
    let res = blocking_post_msg(map, BASE_URL.to_string() + "/verify")?;
    let v: Value = serde_json::from_str(&res)?;
    let res: String = v["session"].to_string().trim_matches('"').to_string();
    Ok(res)
}

pub fn bind_release_verify(
    session_key: &str,
    qq: &str,
    is_release: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("sessionKey", session_key);
    map.insert("qq", qq);
    let res = match is_release {
        true => blocking_post_msg(map, BASE_URL.to_string() + "/release")?,
        false => blocking_post_msg(map, BASE_URL.to_string() + "/bind")?,
    };
    let v: Value = serde_json::from_str(&res)?;
    let res: String = v["msg"].to_string();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_msg() -> Result<(), Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");

        post_msg(map, String::from("http://httpbin.org/post")).await?;
        get_verify()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_verify() -> Result<(), Box<dyn std::error::Error>> {
        // let res = get_verify().await?;
        Ok(())
    }
}
