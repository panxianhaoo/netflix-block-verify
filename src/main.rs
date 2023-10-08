use anyhow::Result;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use url::Url;

const NETFLIX_ADDR: &str = "www.netflix.com";
const SCHEME: &str = "https://";
const PATH: &str = "/title/";
const AREA_AVAILABLE_ID: i32 = 80018499;
const SELF_MADE_AVAILABLE_ID: i32 = 80197526;
const NON_SELF_MADE_AVAILABLE_ID: i32 = 70143836;

#[tokio::main]
async fn main() -> Result<()> {
    let ids = vec![AREA_AVAILABLE_ID, SELF_MADE_AVAILABLE_ID, NON_SELF_MADE_AVAILABLE_ID];
    for i in ids {
        let result = tokio::spawn(check_is_available(i)).await;
    }
    Ok(())
}


async fn check_is_available(id: i32) -> Option<String> {
    let full: String = format!("{SCHEME}{NETFLIX_ADDR}{PATH}{id}");
    let client = reqwest::Client::new();
    let res = client
        .get(full)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36")
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        eprintln!("error");
    }

    let header_map = res.headers();
    let location = header_map.get("x-originating-url");
    if let Some(v) = location {
        let url = Url::parse(v.to_str()?)?;
        let location: Vec<&str> = url.path().split("/").map(|res| res).collect();
        Ok(location[1].to_string())
    }
    None
}
