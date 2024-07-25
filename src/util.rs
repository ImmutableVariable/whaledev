use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serenity::futures::lock::Mutex;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;

// This is a little better ig, i dont think its really great though
static LAST_PASTE_TIME: Lazy<Arc<Mutex<Option<Instant>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn paste(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let last_paste_time = Arc::clone(&LAST_PASTE_TIME);
    let version = env!("CARGO_PKG_VERSION").to_string();
    // paste.com only allows one request every second. This isn't an ideal solution, but I don't really care.
    if let Some(last_time) = last_paste_time.lock().await.as_ref() {
        if last_time.elapsed() < Duration::from_secs(1) {
            sleep(Duration::from_secs(1)).await;
        }
    }

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&format!(
            "Whaledev/{} (+https://github.com/ImmutableVariable/whaledev)",
            version
        ))
        .unwrap(),
    );

    if let Ok(api_key) = std::env::var("DPASTE_API_KEY") {
        // add api key if it exists
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );
    }

    let response = client
        .post("https://dpaste.com/api/")
        .headers(headers)
        .body(format!("content={}", message))
        .send()
        .await?;

    let paste_url = response.headers().get("location").unwrap().to_str()?;

    *LAST_PASTE_TIME.lock().await = Some(Instant::now());

    Ok(paste_url.to_string())
}

pub async fn get_url_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let content = response.text().await?;
    Ok(content)
}

pub fn formatted_number(number: u64) -> String {
    let last_digit = number % 10;
    let last_two_digits = number % 100;
    let number = number.to_string();
    if (11..=13).contains(&last_two_digits) {
        format!("{}th", number)
    } else if last_digit == 1 {
        format!("{}st", number)
    } else if last_digit == 2 {
        format!("{}nd", number)
    } else if last_digit == 3 {
        format!("{}rd", number)
    } else {
        format!("{}th", number)
    }
}

pub fn execute_console_command(code: &str) -> Result<String, std::io::Error> {
    let command = if cfg!(windows) { "cmd" } else { "sh" };
    let output = std::process::Command::new(command)
        .arg("/C")
        .arg(code)
        .output()?;

    // format into a bash code block
    let output_msg = format!("```bash\n{}```", String::from_utf8_lossy(&output.stdout));
    Ok(output_msg)
}

pub fn should_paste_message(message_length: usize) -> bool {
    message_length
        > std::env::var("MAX_MESSAGE_LENGTH")
            .unwrap()
            .parse::<usize>()
            .unwrap()
}
