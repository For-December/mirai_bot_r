use std::collections::HashMap;

use serde_json::json;

use crate::{api::web_utils::ApiParam, setup::conf::APP_CONF};

use super::{baidu_api::get_access_token, web_utils::post_utils};

pub async fn get_text(url: &str) -> String {
    let res = post_utils(ApiParam {
        url: "https://aip.baidubce.com/rest/2.0/ocr/v1/general_basic",
        query: vec![(
            "access_token",
            get_access_token(&APP_CONF.baidu_ocr.api_key, &APP_CONF.baidu_ocr.secret_key)
                .await
                .as_str(),
        )]
        .into_iter()
        .collect(),
        ..Default::default()
    })
    .await
    .unwrap();

    println!("{}", res);

    String::new()
}

#[cfg(test)]
mod test {

    use super::*;
    #[tokio::test]
    async fn test_get_text() {
        get_text("https://th.bing.com/th/id/R.0079a077952dedfe68ca4bb3248443cd?rik=BHNELIZ9czXoHw&riu=http%3a%2f%2fimg.mp.itc.cn%2fupload%2f20170430%2f53814888bc794bf3bb5a8427e5848a57_th.jpeg&ehk=0%2baXa%2f%2flvg7J%2fC4aUyMDC6n6S%2fxMB7anpQkaS%2fyUVxU%3d&risl=&pid=ImgRaw&r=0").await;
    }
}
