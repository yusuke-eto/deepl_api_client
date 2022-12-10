use clap::Parser;
use std::env;

#[derive(Parser)]
struct Cli {
    /// The target text to translate
    text: String,
    /// The target language
    target: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let access_token = env::var("DEEPL_API_ACCESS_TOKEN")
        .expect("Please set environment variable of DEEPL_API_ACCESS_TOKEN");

    let params = [("text", &args.text), ("target_lang", &args.target)];
    let client = reqwest::Client::new();
    let res = client
        .post("https://api-free.deepl.com/v2/translate")
        .form(&params)
        .header("Authorization", format!("DeepL-Auth-Key {}", access_token))
        .send()
        .await
        .unwrap();

    println!("{:?}", res.text().await.unwrap());
}
