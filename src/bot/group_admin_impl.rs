use async_trait::async_trait;
use serde_json::{json, Value};

use super::{bot_trait::GroupAdmin, my_bot::MyBot};

// #[async_trait]
// impl GroupAdmin for MyBot {
//     fn member_admin(&'static self, group_num: &str, member_id: &str, assign: bool) -> String {
//         let json = json!({
//             "target":group_num,
//             "memberId":member_id,
//             "assign":assign,
//         })
//         .to_string();
//         let res = String::new();
//         // let res = super::api_utils::post_msg(json, "/memberAdmin", &self.session_key).unwrap();
//         let res: Value = serde_json::from_str(&res).unwrap();
//         if res["msg"].to_string().eq("\"success\"") {
//             return String::new();
//         }
//         return res["msg"].to_string();
//     }
// }
#[cfg(test)]
mod test {
    use regex::Regex;

    use super::*;
    #[test]
    pub fn test_member_admin() {
        let group_num = "11";
        let member_id = "222";
        let assign = false;
        let json = json!({
            "target":group_num,
            "memberId":member_id,
            "assign":assign,
        })
        .to_string();
        println!("{}", json);
        let reg = Regex::new(r"add admin ([\s\S]+)").unwrap();
        println!("{:#?}", reg);
        for (_, [qq]) in reg
            .captures_iter("add admin 1921567337")
            .map(|c| c.extract())
        {
            println!("{}", qq);
        }
    }
}
