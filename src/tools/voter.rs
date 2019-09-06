use try_from::TryFrom;
use std::collections::HashMap;
use regex::Regex;
use crate::args_parser::{Value};
use crate::methods;
use crate::vk;

struct Answer {
    id: i64,
    text: String,
    votes: i64,
    users: Vec<User>,
}

struct User {
    id: i64,
    first_name: String,
    last_name: String,
}

pub fn execute(
    vk: &vk::VK,
    owner_id: i64,
    poll_id: u64,
    chat_id: u64,
    lang: &String,
)
    -> Result<String, String>
{
    call(&vk, owner_id, poll_id, chat_id, lang)
}

pub fn execute_with_write_to_file(
    vk: &vk::VK,
    owner_id: i64,
    poll_id: u64,
    chat_id: u64,
    lang: &String,
    file: &String,
)
    -> Result<(), String>
{
    std::fs::write(file,
                   call(&vk, owner_id,
                        poll_id, chat_id, lang)?)
        .expect("Can't write to file.");
    Ok(())
}

fn call(
    vk: &vk::VK,
    owner_id: i64,
    poll_id: u64,
    chat_id: u64,
    lang: &String,
)
    -> Result<String, String>
{
    let res = methods::polls_getById::execute(&vk, owner_id, poll_id)?;
    let votes_cnt = res.as_object().unwrap()
        .get("votes").unwrap().as_i64().unwrap();
    const COUNT: i64 = 100i64;
    let circles = (votes_cnt as f64 / COUNT as f64).ceil() as i64;
    let question = res.as_object().unwrap()
        .get("question").unwrap().as_str().unwrap();
    let fields: Vec<&str> = vec!["first_name", "last_name"];
    
    let mut answers: Vec<Answer> = Vec::new();
    for v in res.as_object().unwrap()
        .get("answers").unwrap().as_array().unwrap()
        .iter() {
            answers.push(Answer {
                id: v.as_object().unwrap()
                    .get("id").unwrap().as_i64().unwrap(),
                text: String::from(v.as_object().unwrap()
                    .get("text").unwrap().as_str().unwrap()),
                votes: v.as_object().unwrap()
                    .get("votes").unwrap().as_i64().unwrap(),
                users: Vec::new(),
            });
        };
    let res = methods::messages_getChat::execute(
        &vk, chat_id, lang,
        &vec![String::from("first_name"),
              String::from("last_name")])?;
    let mut not_voted = res.as_object().unwrap()
        .get("users").unwrap().as_array().unwrap()
        .iter()
        .map(|v|
             User {
                 id: v.get("id").unwrap().as_i64().unwrap(),
                 first_name: v.get("first_name").unwrap()
                     .as_str().unwrap().to_string(),
                 last_name: v.get("last_name").unwrap()
                     .as_str().unwrap().to_string()
             })
        .collect::<Vec<_>>();
    let mut voted_but: Vec<User> = Vec::new();
        
    for v in 0..=circles {
        let res = methods::polls_getVoters::execute(
            &vk, owner_id, poll_id,
            &answers.iter()
                .map(|v| v.id)
                .collect::<Vec<i64>>(),
            v * COUNT, &fields, lang)?;
        for v in res.as_array().unwrap().iter() {
            for ans in &mut answers {
                if ans.id == v.as_object().unwrap()
                    .get("answer_id").unwrap().as_i64().unwrap()
                {
                    for v in v.as_object().unwrap()
                        .get("users").unwrap().as_object().unwrap()
                        .get("items").unwrap().as_array().unwrap().iter()
                    {
                        let id = v.as_object().unwrap()
                            .get("id").unwrap().as_i64().unwrap();

                        if not_voted.iter().all(|v| v.id != id) {
                            voted_but.push(User {
                                id: v.as_object().unwrap()
                                    .get("id").unwrap().as_i64().unwrap(),
                                first_name: v.as_object().unwrap()
                                    .get("first_name").unwrap().as_str().unwrap()
                                    .to_string(),
                                last_name: v.as_object().unwrap()
                                    .get("last_name").unwrap().as_str().unwrap()
                                    .to_string()
                        })}
                            
                            
                        not_voted.retain(|v| {
                            v.id != id
                        });
                        ans.users.push(User {
                            id: v.as_object().unwrap()
                                .get("id").unwrap().as_i64().unwrap(),
                            first_name: String::from(v.as_object().unwrap()
                                .get("first_name").unwrap().as_str().unwrap()),
                            last_name: String::from(v.as_object().unwrap()
                                .get("last_name").unwrap().as_str().unwrap())
                        })
                    }
                }
            }
        }
    };
    
    Ok(format!( 
        "voter:
  votes: {VOTES}
  text: \"{TEXT}\"
  id: {ID}
  answers: \n{ANSWERS}
  not_voted:\n{NOT_VOTED}
  voted_but:\n{VOTED_BUT}",
        VOTES = votes_cnt,
        TEXT = question,
        ID = poll_id,
        ANSWERS =
            answers.iter()
            .map(|v| format!(
                "    - answer:
      - id: {ID}
      - text: \"{TEXT}\"
      - votes: {VOTES}
      - users:\n{USERS}",
                ID = v.id,
                TEXT = v.text,
                VOTES = v.votes,
                USERS = v.users
                    .iter()
                    .map(|v| format!(
                        "        - user:
          - id: {ID}
          - name: \"{LAST_NAME} {FIRST_NAME}\"",
                        ID = v.id,
                        LAST_NAME = v.last_name,
                        FIRST_NAME = v.first_name))
                    .collect::<Vec<String>>()
                    .join("\n")))
            .collect::<Vec<String>>()
            .join("\n"),
        NOT_VOTED = not_voted.iter()
            .map(|v| format!(
                "        - user:
          - id: {ID}
          - name: \"{LAST_NAME} {FIRST_NAME}\"",
                ID = v.id,
                LAST_NAME = v.last_name,
                FIRST_NAME = v.first_name))
            .collect::<Vec<_>>()
            .join("\n"),
        VOTED_BUT = voted_but.iter()
            .map(|v| format!(
                "        - user:
          - id: {ID}
          - name: \"{LAST_NAME} {FIRST_NAME}\"",
                ID = v.id,
                LAST_NAME = v.last_name,
                FIRST_NAME = v.first_name))
            .collect::<Vec<_>>()
            .join("\n"),
    ))
}

pub fn parse(map: &mut HashMap<&str, Value>, val: &String)
    -> Result<(), String>
{
    match Regex::new(r"^[+-]?\d+:\d+:\d+:[a-z]+$")
        .unwrap().is_match(val) {
            true => {},
            false =>
                return Err(format!(
                    "Can't check \"{}\" as OwnerID:PollID:ChatID:lang.",
                    val)),
        }
    let vec = val.split(":").collect::<Vec<&str>>();
                 
    match i64::try_from(vec[0]) {
        Ok(v) => map.insert("voter::owner_id", Value::Int(v)),
        Err(_) => return Err(
            format!("Can't parse i64 from \"{}\".", vec[0])),
    };
    match u64::try_from(vec[1]) {
        Ok(v) => map.insert("voter::poll_id", Value::UInt(v)),
        Err(_) => return Err(
            format!("Can't parse u64 from \"{}\".", vec[1])),
    };
    match u64::try_from(vec[2]) {
        Ok(v) => map.insert("voter::chat_id", Value::UInt(v)),
        Err(_) => return Err(
            format!("Can't parse u64 from \"{}\".", vec[2])),
    };
    map.insert("voter::lang", Value::Str(String::from(vec[3])));
    map.insert("voter", Value::Bool(true));
    Ok(())
}
