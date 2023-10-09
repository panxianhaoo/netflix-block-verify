use anyhow::Result;
use colored::Colorize;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use url::Url;
use netflix_block_verify::get_area_name;

const NETFLIX_ADDR: &str = "https://www.netflix.com/title/";
const AREA_AVAILABLE_ID: u32 = 80018499;
const SELF_MADE_AVAILABLE_ID: u32 = 80197526;
const NON_SELF_MADE_AVAILABLE_ID: u32 = 70143836;

#[tokio::main]
async fn main() -> Result<()> {
    let (area, self_made, non_self_made) = tokio::join!(check_is_available(AREA_AVAILABLE_ID),
        check_is_available(SELF_MADE_AVAILABLE_ID),check_is_available(NON_SELF_MADE_AVAILABLE_ID));
    match (area, self_made, non_self_made) {
        (Some(area), Some(_), Some(_)) => {
            println!("{}", "完整解锁，可以看非自制".green());
            println!("{}{}", "Netflix识别地域为".green(), get_area_name(area).green());
        }
        (Some(area), Some(_), None) => {
            println!("{}", "只能看自制".yellow());
            println!("{}{}", "Netflix识别地域为".yellow(), get_area_name(area).yellow());
        }
        (None, None, None) => {
            println!("{}", "无法观看Netflix".red());
        }
        _ => ()
    }
    Ok(())
}


async fn check_is_available(id: u32) -> Option<String> {
    // println!("{:?}", id);
    let full_addr: String = format!("{NETFLIX_ADDR}{id}");
    let client = reqwest::Client::new();
    let res = client
        .get(full_addr)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36")
        .send()
        .await;
    if let Err(_) = &res {
        return None;
    } else {
        let res = res.unwrap();
        if res.status() != StatusCode::OK {
            return None;
        }
        let header_map = res.headers();
        let location = header_map.get("x-originating-url");
        if let Some(v) = location {
            let url = Url::parse(v.to_str().unwrap()).unwrap();
            let location: Vec<&str> = url.path().split("/").map(|res| res).collect();
            return Some(location[1].to_string());
        }
    }
    None
}
