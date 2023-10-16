use std::str::FromStr;

use chatgpt::prelude::{ChatGPT, ModelConfigurationBuilder};

use crate::setup::conf::APP_CONF;
pub trait AI {
    fn process_text(&self, ask: &str) -> String {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let conf = ModelConfigurationBuilder::default()
            .api_url(reqwest::Url::from_str(APP_CONF.gpt_api.end_point.as_str()).unwrap())
            .build()
            .unwrap();

        rt.block_on(async {
            let client = ChatGPT::new_with_config(APP_CONF.gpt_api.api_key.as_str(), conf);

            let resp = client
                .expect("客户端构建失败")
                .send_message(ask)
                .await
                .expect("获取响应失败");
            // println!("resp:{:#?}", resp);

            resp.message().content.clone()
        })
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use chatgpt::prelude::{ChatGPT, ModelConfigurationBuilder};

    use crate::setup::conf::APP_CONF;

    #[test]
    pub fn test_api() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let conf = ModelConfigurationBuilder::default()
            .api_url(reqwest::Url::from_str(APP_CONF.gpt_api.end_point.as_str()).unwrap())
            .build()
            .unwrap();

        rt.block_on(async {
            let client = ChatGPT::new_with_config(APP_CONF.gpt_api.api_key.as_str(), conf);

            let resp = client
                .expect("客户端构建失败")
                .send_message("hello")
                .await
                .expect("获取响应失败");
            // println!("resp:{:#?}", resp);

            println!("Response: {}", resp.message().content);
        });
    }

    #[test]
    pub fn test_api_gpt() {

        // let client = ChatGPT::new_with_config(APP_CONF.gpt_api.api_key.as_str(), conf).unwrap();

        // // Sending a message and getting the completion
        // let response: CompletionResponse = client
        //     .send_message("Describe in five words the Rust programming language.")
        //     .await
        //     .unwrap();

        // println!("Response: {}", response.message().content);
    }
}
