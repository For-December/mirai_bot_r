use base64::{engine::general_purpose, Engine};
use reqwest::StatusCode;
use serde_json::Value;

use crate::setup::conf::APP_CONF;
use std::{collections::HashMap, io::Cursor, process};
pub async fn post_msg(json: String, api_path: &str, session_key: &str) -> Result<String, String> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let res = reqwest::Client::new()
        .post(&(APP_CONF.base_url.clone() + api_path))
        .body(json.clone())
        .header("sessionKey", session_key)
        .send()
        .await
        .unwrap_or_else(|err| {
            println!("POST request error: {err}");
            println!("POST url is {}", APP_CONF.base_url.clone() + api_path);
            process::exit(0);
        });

    match res.status() {
        StatusCode::OK => {
            let res = res.text().await.unwrap();
            let resp_json: Value = serde_json::from_str(&res).unwrap();
            let code = resp_json["code"].to_string();
            println!("code {}", code);
            if code.is_empty() || code.eq("0") {
                Ok(res)
            } else {
                Err(format!("error: {}", resp_json["msg"].to_string()))
            }
        }
        code => Err(format!("RESPONSE error code: {}", code)),
    }
    // println!("{:#?}", res);
}

pub async fn get_msg(
    map: HashMap<&str, &str>,
    api_path: &str,
    session_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let mut req_builder = reqwest::Client::builder()
        .no_proxy()
        .build()
        .unwrap()
        .get(&(APP_CONF.base_url.clone() + api_path))
        .header("sessionKey", session_key);
    for ele in map {
        req_builder = req_builder.query(&[ele]);
    }
    let res = req_builder.send().await?.text().await?;
    // println!("{:#?}", res);

    Ok(res)
}

pub async fn get_bytes_with_body(url: &str, body: String) -> Result<String, String> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let req_builder = reqwest::Client::builder()
        .no_proxy()
        .build()
        .unwrap()
        .post(url)
        .body(body);
    let resp = req_builder.send().await.unwrap();
    if resp.status().is_success() {
        let image_byte = resp.bytes().await.unwrap();
        let img = image::load_from_memory(&image_byte).unwrap();
        img.resize(
            img.width() / 3,
            img.height() / 3,
            image::imageops::FilterType::CatmullRom,
        );

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();
        let base64_img = general_purpose::STANDARD_NO_PAD.encode(bytes);
        Ok(base64_img)
    } else {
        Err(format!("{:#?}", resp))
    }
}
pub async fn get_bytes(url: &str) -> Result<String, String> {
    // println!("{}", APP_CONF.base_url.clone() + api_path);
    let req_builder = reqwest::Client::new().get(url);
    let resp = req_builder.send().await.unwrap();
    if resp.status().is_success() {
        let image_byte = resp.bytes().await.unwrap();
        let img = image::load_from_memory(&image_byte).unwrap();
        img.resize(
            img.width() / 3,
            img.height() / 3,
            image::imageops::FilterType::CatmullRom,
        );
        // img.save("a.png").unwrap();

        // let mut buffer = Cursor::new(Vec::new());
        // img.write_to(&mut buffer, image::ImageOutputFormat::Png)
        //     .unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();
        let base64_img = general_purpose::STANDARD_NO_PAD.encode(bytes);
        Ok(base64_img)
    } else {
        Err(format!("{:#?}", resp))
    }
    // println!("{:#?}", res);
}

#[cfg(test)]
pub mod test {
    use super::{get_bytes, get_bytes_with_body};

    #[tokio::test]
    async fn test_base64() {
        println!(r#""base64":"\s+""#);
        let res = get_bytes("https://whatslink.info/image/6d89f8c9437ac9b9824b4ca1020db1a8")
            .await
            .unwrap();
        println!("{}", res);

        let res = get_bytes_with_body(
            "http://localhost:9876/screen",
            String::from("https://www.baidu.com"),
        )
        .await
        .unwrap();
        println!("{}", res);
    }
}
