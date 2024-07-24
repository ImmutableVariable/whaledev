use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// This is a terrible way i think to handle this, but I don't really care.
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
    if last_two_digits >= 11 && last_two_digits <= 13 {
        return format!("{}th", number);
    } else if last_digit == 1 {
        return format!("{}st", number);
    } else if last_digit == 2 {
        return format!("{}nd", number);
    } else if last_digit == 3 {
        return format!("{}rd", number);
    } else {
        return format!("{}th", number);
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
    return Ok(output_msg);
}

pub fn should_paste_message(message_length: usize) -> bool {
    message_length > std::env::var("MAX_MESSAGE_LENGTH").unwrap().parse::<usize>().unwrap()
}