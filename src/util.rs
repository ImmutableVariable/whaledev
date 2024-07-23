use std::time::{Duration, Instant};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use tokio::time::sleep;

//... this is just... probably horrible. However, it compiles... so... I guess it's fine?
static mut LAST_PASTE_TIME: Option<Instant> = None;

pub async fn paste(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let last_paste_time = unsafe { LAST_PASTE_TIME };
    let version = env!("CARGO_PKG_VERSION").to_string();
    // paste.com only allows one request every second. This isn't an ideal solution, but I don't really care.
    if let Some(last_time) = last_paste_time {
        if last_time.elapsed() < Duration::from_secs(1) {
            sleep(Duration::from_secs(1)).await;
        }
    }

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert(USER_AGENT, HeaderValue::from_str(&format!("Whaledev/{} (+https://github.com/ImmutableVariable/whaledev)", version)).unwrap());
    
    let response = client.post("https://dpaste.com/api/")
        .headers(headers)
        .body(format!("content={}", message))
        .send()
        .await?;
    
    let paste_url = response.headers().get("location").unwrap().to_str()?;
    
    unsafe { LAST_PASTE_TIME = Some(Instant::now()); }
    Ok(paste_url.to_string())

}