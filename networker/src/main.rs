use reqwest::Response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://webhook.site/6179deb1-c495-44c0-ad1e-2041b9a47abd";
    let response: Response = reqwest::get(URL).await?;
    let result: serde_json::Value = response.json().await?;
    println!("{:#?}", result);
    Ok(())
}
