use std::{
    fs,
    io::{self, Write},
};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut tokenlist = String::new();
    print!("Tokens list: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tokenlist).unwrap();
    let tokens = fs::read_to_string(tokenlist.trim()).unwrap();
    loop {
        for token in tokens.split("\n") {
            let token = token.trim();
            tokio::spawn(check_token(client.clone(), String::from(token)));
        }
    }
}
async fn check_token(client: reqwest::Client, token: String) {
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