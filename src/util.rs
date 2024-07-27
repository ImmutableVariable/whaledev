use chrono::{NaiveDateTime, TimeDelta};
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serenity::futures::lock::Mutex;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;

// This is a little better ig, i dont think its really great though
static LAST_PASTE_TIME: Lazy<Arc<Mutex<Option<Instant>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn paste(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let last_paste_time = Arc::clone(&LAST_PASTE_TIME);

    // paste.com only allows one request every second. This isn't an ideal solution, but I don't really care.
    if let Some(last_time) = last_paste_time.lock().await.as_ref() {
        if last_time.elapsed() < Duration::from_secs(1) {
            sleep(Duration::from_secs(1)).await;
        }
    }

    let params = [("content", message)];
    let client = reqwest::Client::new();
    let res = client
        .post("https://dpaste.com/api/")
        .form(&params)
        .send()
        .await?;

    let paste_url = &res.headers()["location"];

    *LAST_PASTE_TIME.lock().await = Some(Instant::now());

    Ok(paste_url.to_str().unwrap().to_string())
}

pub async fn get_url_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let content = response.text().await?;
    Ok(content)
}

/// Formats a number into a human readable ordinal number, e.g. 1st, 2nd, 3rd, 4th, 5th, etc.
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

/// Returns the time elapsed from a string in the format of "%Y-%m-%d %H:%M:%S"
/// The expected input is from the database's built in timestamp
pub fn time_elapsed_from_string(time_string: &str) -> Result<TimeDelta, Box<dyn std::error::Error>> {
    let last_message_at = NaiveDateTime::parse_from_str(time_string, "%Y-%m-%d %H:%M:%S")?;
    let now = chrono::Utc::now().naive_utc();
    let duration = now - last_message_at;
    Ok(duration)
}

/// calculate the xp required to reach the next level
/// The formula is `xp_constant * level^2 + xp_constant`
/// The xp_constant is the value required to reach level 1
pub fn xp_required(level: i32, xp_constant: i32) -> i32 {
    xp_constant * (level * level) + xp_constant
}
