use std::{env, fs, vec};
use std::sync::Arc;
use futures::future::join_all;
use tokio::task::JoinHandle;
#[tokio::main]
async fn main() {
    let mut tasks: Vec<JoinHandle<()>> = vec![];
    let args: Vec<String> = env::args().collect();
    let client = Arc::new(reqwest::Client::new());
    let tokenlist = args.get(1).unwrap();
    let tokens = fs::read_to_string(tokenlist.trim()).unwrap();

    for token in tokens.split("\n") {
        let token = token.trim();
        tasks.push(tokio::spawn(check_token(
            client.clone(),
            String::from(token),
        )));
    }
    join_all(tasks).await;
}
async fn check_token(client: Arc<reqwest::Client>, token: String) {
    let resp = client
        .get("https://discord.com/api/v9/users/@me")
        .header("Authorization", &token)
        .send()
        .await
        .unwrap();
    if resp.status().is_success() {
        println!("{} is valid", token);
    } else {
        println!("{} is invalid", token)
    }
}
