use clipboard_win::{formats, get_clipboard};
use reqwest;
use clap::{Parser};

/// Push clipboard content to iOS
#[derive(Parser, Debug)]
struct Args {
    /// target key
    #[clap(short, long)]
    key: String,
}


#[tokio::main]
async fn main() {
    let context: String = match get_clipboard(formats::Unicode) {
        Ok(result) => result,
        Err(_) => String::from("剪贴板为空"),
    };
    // println!("{:?}", context);

    let args = Args::parse();
    let url = format!(
        "https://home.promarcus.com:8100/{}/{}?copy={}&autoCopy=1&level=timeSensitive&isArchive=1",
        args.key, &context, &context
    );

    reqwest::get(url).await.unwrap();
}
