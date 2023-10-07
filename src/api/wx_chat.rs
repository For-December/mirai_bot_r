use crate::bot::api_utils::post_msg;

fn get_access_token() -> String {
    // post_msg(json, api_path, session_key);
    return String::from("value");
}
pub trait AI {
    fn process_text(&self, ask: &str, answer: &str) {}
}
