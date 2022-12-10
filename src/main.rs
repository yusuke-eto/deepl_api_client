use std::env;

#[tokio::main]
async fn main() {
    let access_token = env::var("DEEPL_API_ACCESS_TOKEN")
        .expect("Please set environment variable of DEEPL_API_ACCESS_TOKEN");

    let params = [("text", "good morning"), ("target_lang", "JA")];
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
