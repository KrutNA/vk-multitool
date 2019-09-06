use std::collections::HashMap;
use serde_json as json;
use crate::vk;

pub fn execute(
    vk: &vk::VK,
    chat_id: u64,
    lang: &String,
    fields: &Vec<String>
)
    -> Result<json::Value, String>
{
    let method = "messages.getChat";
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("chat_id", chat_id.to_string());
    map.insert("lang", lang.clone());
    map.insert("fields", fields.join(","));
    vk.call_with_args(method, &map)
}
