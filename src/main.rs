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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context: String = get_clipboard(formats::Unicode).expect("No result");
    // println!("{:?}", context);

    let args = Args::parse();
    let url = format!(
        "https://home.promarcus.com:8100/{}/{}?copy={}&autoCopy=1&level=timeSensitive&isArchive=1",
        args.key, &context, &context
    );

    let resp = reqwest::get(url).await?.text().await?;
    // println!("{:#?}", resp);
    Ok(())
}
