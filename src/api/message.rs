use serde::{Deserialize, Serialize};

enum MessageType {
    Plain(String),
    Image(String),
    Source((i64, i64)),
    At(i64),
}

#[derive(Debug)]
pub struct MessageChain {
    message_chain: Vec<Message>,
}
impl MessageChain {
    // 链式调用，所有权转移
    pub fn new() -> MessageChain {
        let message_chain: Vec<Message> = Vec::new();
        return MessageChain { message_chain };
    }
    pub fn from(message_chain: Vec<Message>) -> MessageChain {
        return MessageChain { message_chain };
    }
    pub fn build_img(mut self, url: String) -> Self {
        // Message::with(String::from("value"));
        self.message_chain
            .push(Message::with(MessageType::Image(url)));

        self
    }
    pub fn build_text(mut self, text: String) -> Self {
        self.message_chain
            .push(Message::with(MessageType::Plain(text)));
        self
    }
    pub fn build_at(mut self, target: String) -> Self {
        self.message_chain.push(Message::with(MessageType::At(
            target.parse::<i64>().expect("解析艾特的 qq 号失败！"),
        )));
        self
    }

    pub fn get_message_chain(&self) -> &Vec<Message> {
        &self.message_chain
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub _type: String, // [Plain, Image, Source]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

trait With<T> {
    fn with(value: T) -> Self;
}
impl With<MessageType> for Message {
    fn with(value: MessageType) -> Self {
        match value {
            MessageType::Plain(text) => Message {
                _type: String::from("Plain"),
                text: Some(text),
                ..Default::default()
            },
            MessageType::Image(url) => Message {
                _type: String::from("Image"),
                url: Some(url),
                ..Default::default()
            },
            MessageType::Source((id, time)) => Message {
                _type: String::from("Source"),
                id: Some(id),
                time: Some(time),
                ..Default::default()
            },
            MessageType::At(target) => Message {
                _type: String::from("At"),
                target: Some(target),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_post_msg() -> Result<(), Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");

        // _post_msg(map, String::from("http://httpbin.org/post")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_verify() -> Result<(), Box<dyn std::error::Error>> {
        // let res = get_verify().await?;
        Ok(())
    }
}
