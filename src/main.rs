use anyhow::Result;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use url::Url;

const NETFLIX_ADDR: &str = "https://www.netflix.com/title/";
const AREA_AVAILABLE_ID: u32 = 80018499;
const SELF_MADE_AVAILABLE_ID: u32 = 80197526;
const NON_SELF_MADE_AVAILABLE_ID: u32 = 70143836;

#[tokio::main]
async fn main() -> Result<()> {
    let (area, self_made, non_self_made) = tokio::join!(check_is_available(AREA_AVAILABLE_ID),
        check_is_available(SELF_MADE_AVAILABLE_ID),check_is_available(NON_SELF_MADE_AVAILABLE_ID));
    if let (Some(_), Some(_), Some(_)) = (area, self_made, non_self_made) {
        println!("OK");
    }
    Ok(())
}


async fn check_is_available(id: u32) -> Option<String> {
    println!("{:?}", id);
    let full: String = format!("{NETFLIX_ADDR}{id}");
    let client = reqwest::Client::new();
    let res = client
        .get(full)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36")
        .send()
        .await.unwrap();

    if res.status() != StatusCode::OK {
        return None;
    }

    let header_map = res.headers();
    let location = header_map.get("x-originating-url");
    if let Some(v) = location {
        let url = Url::parse(v.to_str().unwrap()).unwrap();
        let location: Vec<&str> = url.path().split("/").map(|res| res).collect();
        // println!("{:?}", location[1].to_string());
        return Some(location[1].to_string());
    }
    None
}
