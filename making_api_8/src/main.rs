use error_chain::error_chain;
use reqwest::header::USER_AGENT;
use serde::Deserialize;


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    login: String,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let request_url: String = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response = client
        .get(&request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;

    println!("Users Data | {:#?}", users);

    Ok(())
}
