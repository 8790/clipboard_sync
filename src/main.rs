#![windows_subsystem = "windows"]

use std::env;
use clipboard_win::{formats, get_clipboard};
use reqwest;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    
    let key = &args[1];

    let context: String = match get_clipboard(formats::Unicode) {
        Ok(result) => result,
        Err(_) => String::from("剪贴板为空"),
    };
    // println!("{:?}", context);

    let url = format!(
        "https://home.promarcus.com:8100/{}/{}?copy={}&autoCopy=1&level=timeSensitive&isArchive=1",
        key, &context, &context
    );

    reqwest::get(url).await.unwrap();
}
