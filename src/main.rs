#![windows_subsystem = "windows"]

use std::{env, collections::HashMap};
use clipboard_win::{formats, get_clipboard};
use reqwest;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    
    let key = &args[1];

    let context = match get_clipboard(formats::Unicode) {
        Ok(result) => result,
        Err(_) => String::from("剪贴板为空"),
    };

    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("device_key", &key);
    body.insert("title", "Windows 剪贴板");
    body.insert("body", &context);
    body.insert("icon", "https://home.promarcus.com:8800/static/images/icons/airdrop.ico");
    body.insert("automaticallyCopy", "1");
    body.insert("level", "timeSensitive");
    body.insert("isArchive", "1");
    body.insert("copy", &context);
    body.insert("url", &context);

    let client = reqwest::Client::new();

    client.post("https://home.promarcus.com:8100/push")
    .json(&body).send().await.unwrap();
}
