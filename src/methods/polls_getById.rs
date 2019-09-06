use std::collections::HashMap;
use serde_json as json;
use crate::vk;

pub fn execute(
    vk: &vk::VK,
    owner_id: i64,
    poll_id: u64,
)
    -> Result<json::Value, String>
{
    let method = "polls.getById";
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("owner_id", owner_id.to_string());
    map.insert("poll_id", poll_id.to_string());
    vk.call_with_args(method, &map)
}
