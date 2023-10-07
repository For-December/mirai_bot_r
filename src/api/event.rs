use serde_json::Value;

use super::{message::MessageChain, group::GroupSender};

pub enum Event {
    GroupEvent((MessageChain, GroupSender)),
    NudgeEvent((String, String, Value)),
}
