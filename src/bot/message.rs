use serde::{Deserialize, Serialize};

enum MessageType {
    Plain(String),
    Image(String),
    Source((Identity, i64)),
    At(i64),
    Voice(String),
}

#[derive(Debug, Clone)]
pub struct MessageChain {
    pub group_num: Option<String>,
    message_chain: Vec<Message>,
}

impl MessageChain {
    // 链式调用，所有权转移
    pub fn new() -> MessageChain {
        let message_chain: Vec<Message> = Vec::new();
        return MessageChain {
            group_num: None,
            message_chain,
        };
    }
    pub fn from(group_num: Option<String>, message_chain: Vec<Message>) -> MessageChain {
        return MessageChain {
            group_num,
            message_chain,
        };
    }
    pub fn build_target(mut self, group_num: &str) -> Self {
        self.group_num = Some(String::from(group_num));
        self
    }
    pub fn ref_build_img(&mut self, url: String) -> &Self {
        // Message::with(String::from("value"));
        self.message_chain
            .push(Message::with(MessageType::Image(url)));
        self
    }
    pub fn ref_build_base64_img(&mut self, base_64: String) -> &Self {
        // Message::with(String::from("value"));
        self.message_chain
            .push(Message::with(MessageType::Image(base_64)));
        self
    }
    pub fn build_img(mut self, url: String) -> Self {
        // Message::with(String::from("value"));
        self.message_chain
            .push(Message::with(MessageType::Image(url)));

        self
    }
    pub fn ref_build_text(&mut self, text: &str) -> &Self {
        self.message_chain
            .push(Message::with(MessageType::Plain(String::from(text))));
        self
    }
    pub fn build_text(mut self, text: &str) -> Self {
        self.message_chain
            .push(Message::with(MessageType::Plain(String::from(text))));
        self
    }
    pub fn build_at(mut self, target: String) -> Self {
        self.message_chain.push(Message::with(MessageType::At(
            target.parse::<i64>().expect("解析艾特的 qq 号失败！"),
        )));
        self
    }
    // 独占，无法联合
    pub fn build_voice(mut self, path: &str) -> Self {
        self.message_chain
            .push(Message::with(MessageType::Voice(String::from(path))));

        self
    }

    pub fn get_message_chain(&self) -> &Vec<Message> {
        &self.message_chain
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Identity {
    Integer(i64),
    Str(String),
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
    #[serde(rename = "imageId")]
    pub image_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self {
            _type: self._type.clone(),
            text: self.text.clone(),
            url: self.url.clone(),
            id: self.id.clone(),
            time: self.time.clone(),
            target: self.target.clone(),
            display: self.display.clone(),
            path: self.path.clone(),
            image_id: self.image_id.clone(),
            base64: self.base64.clone(),
        }
    }
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
            MessageType::Image(param) => {
                if param.contains("}.") {
                    let image_id = param;
                    Message {
                        _type: String::from("Image"),
                        image_id: Some(image_id),
                        ..Default::default()
                    }
                } else if param.contains("http") {
                    let url = param;
                    Message {
                        _type: String::from("Image"),
                        url: Some(url),
                        ..Default::default()
                    }
                } else {
                    let base64 = param;
                    Message {
                        _type: String::from("Image"),
                        base64: Some(base64),
                        ..Default::default()
                    }
                }
            }
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
            MessageType::Voice(path) => Message {
                _type: String::from("Voice"),
                path: Some(path),
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
