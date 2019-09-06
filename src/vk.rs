use std::collections::HashMap;
use serde_json as json;
use reqwest::{Client};

pub struct VK
{
    API_URL: String,
    TOKEN: String,
    VERSION: String,
    client: Client,
}
impl VK
{
    pub fn new(token: &str) -> Self {
        return VK {
            API_URL: String::from("https://api.vk.com/method"),
            TOKEN: String::from(token),
            VERSION: String::from("5.101"),
            client: Client::new(),
        }
    }
    pub fn new_with_version(token: &str, version: &str) -> Self {
        return VK {
            API_URL: String::from("https://api.vk.com/method"),
            TOKEN: String::from(token),
            VERSION: String::from(version),
            client: Client::new(),
        }
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    fn request(&self, url: &String) -> Result<json::Value, String> {
        let res: json::Value = self.client.get(&url[..]).send()
            .expect("Can't access to VK API.")
            .json().unwrap();
        if res.as_object().unwrap().contains_key("error") {
            Err(format!("\"{}\". Request params: [{}]",
                            res.as_object().unwrap().get("error").unwrap()
                            .as_object().unwrap().get("error_msg").unwrap()
                            .as_str().unwrap(),
                            res.as_object().unwrap().get("error").unwrap()
                            .as_object().unwrap().get("request_params").unwrap()
                            .as_array().unwrap().iter().map(|param| {
                                format!("{}:{}",
                                        param.as_object().unwrap().get("key").unwrap()
                                        .as_str().unwrap(),
                                        param.as_object().unwrap().get("value").unwrap()
                                        .as_str().unwrap()
                                )
                            }).collect::<Vec<String>>().join(", ")))
        } else {
            Ok(res.as_object().unwrap().get("response").unwrap().to_owned())
        }
    }
    pub fn call(&self, method: &str) -> Result<json::Value, String> {
        let url =
            format!("{API}/{METHOD}?access_token={TOKEN}&v={VERSION}",
                    API = self.API_URL,
                    METHOD = method,
                    TOKEN = self.TOKEN,
                    VERSION = self.VERSION);
        self.request(&url)
    }
    pub fn call_with_args(
        &self,
        method: &str,
        args: &HashMap<&str, String>,  
    )
        -> Result<json::Value, String> {
        let url =
            format!("{API}/{METHOD}?access_token={TOKEN}&v={VERSION}&{OTHER_ARGS}",
                    API = self.API_URL,
                    METHOD = method,
                    TOKEN = self.TOKEN,
                    VERSION = self.VERSION,
                    OTHER_ARGS = args.iter()
                    .map(|(k,v)|
                         format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&"));
        self.request(&url)
    }
}
