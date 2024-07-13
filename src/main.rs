use std::{
    fs,
    io::{self, Write},
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut tokenlist = String::new();
    print!("Tokens list: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut tokenlist)
        .expect("Failed to read token input");
    let tokens = fs::read_to_string(tokenlist.trim()).unwrap();
    for token in tokens.split("\n") {
        let token = token.trim();
        let resp = client
            .get("https://discord.com/api/v9/users/@me")
            .header("Authorization", token)
            .send()
            .await?;
        if resp.status().is_success() {
            println!("{} is valid", token);
        } else {
            println!("{} is invalid", token);
        }
    }
    Ok(())
}
