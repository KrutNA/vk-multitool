use std::collections::HashMap;
use serde_json as json;
use crate::vk;

pub fn execute(
    vk: &vk::VK,
    owner_id: i64,
    poll_id: u64,
    answer_ids: &Vec<i64>,
    offset: i64,
    fields: &Vec<&str>,
    lang: &String,
)
    -> Result<json::Value, String>
{
    let method = "polls.getVoters";
    let mut args: HashMap<&str, String> = HashMap::new();
    args.insert("owner_id", owner_id.to_string());
    args.insert("poll_id", poll_id.to_string());
    args.insert("answer_ids", answer_ids.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","));
    args.insert("offset", offset.to_string());
    args.insert("fields", fields.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","));
    args.insert("lang", lang.clone());
    vk.call_with_args(method, &args)   
}
