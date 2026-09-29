use std::io;

use anyhow::Result;
use colored::Colorize;
use reqwest::header::USER_AGENT;
use reqwest::{Client, StatusCode};

use area::get_area_name;

mod area;

const NETFLIX_ADDR: &str = "https://www.netflix.com";
const SELF_MADE_AVAILABLE_ID: u32 = 80197526;
const NON_SELF_MADE_AVAILABLE_ID: u32 = 70143836;
const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36";

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let (self_made, non_self_made) = tokio::join!(
        check_is_available(&client, SELF_MADE_AVAILABLE_ID),
        check_is_available(&client, NON_SELF_MADE_AVAILABLE_ID)
    );
    match (&self_made, non_self_made.is_some()) {
        (Some(area), true) => {
            println!("{}", "完整解锁，可以看非自制".green());
            println!("{}", format!("Netflix识别地域为{}", get_area_name(area)).green());
        }
        (Some(area), false) => {
            println!("{}", "只能看自制".yellow());
            println!("{}", format!("Netflix识别地域为{}", get_area_name(area)).yellow());
        }
        (None, _) => {
            println!("{}", "无法观看Netflix".red());
        }
    }
    io::stdin().read_line(&mut String::new())?;
    Ok(())
}

/// 请求 title 页并跟随重定向（如 301 到 /sg/title/xxx），解锁时最终 200，
/// 地区码直接从最终 URL 路径解析
async fn check_is_available(client: &Client, id: u32) -> Option<String> {
    let url = format!("{NETFLIX_ADDR}/title/{id}");
    let res = client
        .get(url)
        .header(USER_AGENT, BROWSER_USER_AGENT)
        .send()
        .await
        .ok()?;
    if res.status() != StatusCode::OK {
        return None;
    }
    let segment = res.url().path().split('/').nth(1)?; // "/sg/title/xxx" -> "sg"
    let code = segment.split('-').next().filter(|c| !c.is_empty())?;
    Some(code.to_string())
}
