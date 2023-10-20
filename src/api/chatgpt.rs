use std::{str::FromStr, sync::Arc, time::Duration};

use async_trait::async_trait;
use chatgpt::prelude::{ChatGPT, Conversation, ModelConfigurationBuilder};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use crate::setup::conf::APP_CONF;

lazy_static! {
    static ref GPT_CONTENT: Arc<Mutex<Conversation>> = {
        let conf = ModelConfigurationBuilder::default()
            .api_url(reqwest::Url::from_str(APP_CONF.gpt_api.end_point.as_str()).unwrap())
            .timeout(Duration::from_secs(100))
            .build()
            .unwrap();
        let client = ChatGPT::new_with_config(APP_CONF.gpt_api.api_key.as_str(), conf).unwrap();
        Arc::new(Mutex::new(client.new_conversation()))
    };
}

#[async_trait]
pub trait AI {
    async fn process_text(user_id: &str, ask: &str) -> String {
        let content = Arc::clone(&GPT_CONTENT);
        let resp = content.try_lock_owned();
        if resp.is_err() {
            return String::from("别急，让我思考一会儿~");
        }
        // .send_message(ask).await.unwrap();
        resp.unwrap()
            .send_message(ask)
            .await
            .unwrap()
            .message()
            .content
            .clone()
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
