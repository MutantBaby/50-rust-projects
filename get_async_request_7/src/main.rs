use error_chain::error_chain;

error_chain! {
        foreign_links {
        Io(std::io::Error) #[cfg(unix)];
        HttpRequest(reqwest::Error);
    }

}

#[tokio::main]
async fn main() -> Result<()> {
    let response: reqwest::Response = reqwest::get("http://httpbin.org/get").await?;
    println!("Header | {:#?}", response.headers());
    println!("Status | {}", response.status());

    let body: String = response.text().await?;
    println!("Body | {}", body);

    Ok(())
}
