use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("Request URL: {}", request_url);

    let client = reqwest::Client::new();

    let res = client.get(&request_url).header(USER_AGENT, "reqwest").send().await?;
    println!("Status: {}", res.status());

    let users: Vec<User> = res.json().await?;
    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
