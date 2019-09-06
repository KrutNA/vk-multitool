use clap::{App, Arg, ArgMatches};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader, Yaml};
use crate::tools;

#[derive(Clone, Debug)]
pub enum Value
{
    UInt(u64),
    Int(i64),
    Str(String),
    Bool(bool),
}
pub trait Convert
{
    fn as_u64(&self) -> Result<u64, String>;
    fn as_i64(&self) -> Result<i64, String>;
    fn as_str(&self) -> Result<String, String>;
    fn as_bool(&self) -> Result<bool, String>;
}
impl Convert for Value
{
    fn as_u64(&self) -> Result<u64,String> {
        match self {
            Value::UInt(v) => Ok(*v),
            v => Err(format!("Value \"{:?}\" is not an u64.", v)),
        }
    }
    fn as_i64(&self) -> Result<i64,String> {
        match self {
            Value::Int(v) => Ok(*v),
            v => Err(format!("Value \"{:?}\" is not an i64.", v)),
        }
    }
    fn as_str(&self) -> Result<String, String> {
        match self {
            Value::Str(v) => Ok(v.clone()),
            v => Err(format!("Value \"{:?}\" is not a String.", v)),
        }
    }
    fn as_bool(&self) -> Result<bool, String> {
        match self {
            Value::Bool(v) => Ok(*v),
            v => Err(format!("Value \"{:?}\" is not a bool.", v)),
        }
    }
}

pub fn parse(
    map: &mut HashMap<&str, Value>
)
    -> Result<(), String>
{
    let matches = App::new("VK multitool")
        .version("0.1")
        .author("Krutko Nikita / KrutNA <krutna@pm.me>")
        .about("Multitool for VK.com. Currently contains:")
        .arg(Arg::with_name("config")
             .short("c").long("config")
             .help("Sets custom configuration file")
             .takes_value(true)
             .value_name("FILE")
             .default_value("config.yml")
             .validator(|name| {
                 match File::open(&name) {
                     Ok(_) => Ok(()),
                     Err(_) => Err(
                         format!("Configuration file \"{}\" not found", name)),
                 }
             }))
        .arg(Arg::with_name("out")
             .long("out")
             .help("Sets output file")
             .takes_value(true)
             .value_name("FILE"))
        .arg(Arg::with_name("voter")
             .long("voter")
             .help("Sets owner id, poll id and language for users info")
             .takes_value(true)
             .value_name("OwnerID:PollID:ChatID:lang")
             .validator(|val| {
                 if Regex::new(r"^[+-]?\d+:\d+:\d+:[a-z]+$")
                     .unwrap().is_match(&val)
                 {
                     Ok(())
                 } else {
                     Err(format!(
                         "Can't check \"{}\" as OwnerID:PollID:ChatID:lang.",
                         val))
                 }
             }))
        .get_matches();
    let config = matches.value_of("config").unwrap_or("config.yml");
    match parse_yaml(map, &config) {
        Ok(()) => {},
        Err(v) => return Err(v),
    };
    match parse_matches(map, &matches) {
        Ok(()) => {},
        Err(v) => return Err(v),
    };
    Ok(())
}

fn parse_yaml(
    map: &mut HashMap<&str, Value>,
    config: &str
)
    -> Result<(), String>
{
    match File::open(config) {
        Ok(mut f) => {
            let mut v = String::new();
            f.read_to_string(&mut v).unwrap();
            let doc = match YamlLoader::load_from_str(&v) {
                Ok(v) => v,
                Err(_) => return Err(format!("Can't load config \"{}\"", config)),
            };
            match doc[0]["token"] {
                Yaml::String(ref s) => {
                    map.insert("token", Value::Str(s.clone()));
                },
                _ => return Err(String::from("Token in config file required!")),
            }
            match doc[0]["voter"] {
                Yaml::String(_) => {
                    tools::voter::parse(
                        map,
                        &String::from(doc[0]["voter"].as_str().unwrap()))?;
                },
                _ => {},
            };
        },
        Err(_) => return Err(format!("Can't open file \"{}\".", config)),
    };
    Ok(())
}

fn parse_matches(
    map: &mut HashMap<&str, Value>,
    matches: &ArgMatches
)
    -> Result<(), String>
{
    if matches.is_present("voter") {
        tools::voter::parse(
            map,
            &String::from(matches.value_of("voter").unwrap()))?;
    }
    if matches.is_present("out") {
        map.insert(
            "out",
            Value::Str(matches.value_of("out").unwrap().to_string()));
    }
    Ok(())
}
