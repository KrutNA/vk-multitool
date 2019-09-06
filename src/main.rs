mod args_parser;
mod vk;
mod methods;
mod tools;
use args_parser::{Value, Convert};
use std::collections::HashMap;
use termcolor::{WriteColor, Color, StandardStream, ColorChoice, ColorSpec};

fn main()
{
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut map: HashMap<&str, Value> = HashMap::new();
    match args_parser::parse(&mut map) {
        Ok(()) => {},
        Err(v) => return print_error(&mut stdout, &v),
    }
    let api = vk::VK::new(&map.get("token").unwrap().as_str().unwrap());
    match call(&mut map, &api) {
        Ok(()) => {},
        Err(v) => return print_error(&mut stdout, &v),
    };
}

fn print_error(stdout: &mut StandardStream, err: &String)
{
    stdout.set_color(ColorSpec::new()
        .set_fg(Some(Color::Rgb(255, 106, 107)))
        .set_bold(true))
        .unwrap();
    print!("error");
    stdout.reset().unwrap();
    println!(": {}", err);
}

fn call(
    map: &mut HashMap<&str, Value>,
    api: &vk::VK
)
    -> Result<(), String>
{
    if map.contains_key("voter") {
        call_voter(map, &api)?;
    };
    Ok(())
}

fn call_voter(
    map: &mut HashMap<&str, Value>,
    api: &vk::VK
)
    -> Result<(), String>
{
    if map.contains_key("out") {
        match tools::voter::execute_with_write_to_file(
            &api,
            map.get("voter::owner_id").unwrap().as_i64().unwrap(),
            map.get("voter::poll_id").unwrap().as_u64().unwrap(),
            map.get("voter::chat_id").unwrap().as_u64().unwrap(),
            &map.get("voter::lang").unwrap().as_str().unwrap(),
            &map.get("out").unwrap().as_str().unwrap(),
        ) {
            Ok(()) => {},
            Err(v) => return Err(v),
        }
    } else {
        match tools::voter::execute(
            &api,
            map.get("voter::owner_id").unwrap().as_i64().unwrap(),
            map.get("voter::poll_id").unwrap().as_u64().unwrap(),
            map.get("voter::chat_id").unwrap().as_u64().unwrap(),
            &map.get("voter::lang").unwrap().as_str().unwrap(),
        ) {
            Ok(v) => println!("{}", v),
            Err(v) => return Err(v),
        }
    };
    Ok(())
}
