use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub permission: String,
}

pub struct GroupSender {
    sender: Value, // messageChain and sender
}
impl GroupSender {
    pub fn new(sender: Value) -> GroupSender {
        GroupSender { sender }
    }
    pub fn get_id(&self) -> String {
        self.sender["id"].to_string()
    }
    pub fn get_member_name(&self) -> String {
        self.sender["memberName"].to_string()
    }
    pub fn get_special_title(&self) -> String {
        self.sender["specialTitle"].to_string()
    }
    pub fn get_group(&self) -> Group {
        println!("{:#?}", self.sender);
        let res: Group = serde_json::from_value(self.sender["group"].clone()).unwrap();
        res
    }
}
