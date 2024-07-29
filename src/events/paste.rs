use serenity::{all::{Context, Message}, futures};

use crate::util;

/// Handles pasting a file to a paste service.
/// THE MESSAGE MUST CONTAIN A ATTACHMENT!
pub async fn paste_file_handler(ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    let formatted_content = msg
        .attachments
        .iter()
        .filter(|&attachment| attachment
            .content_type
            .as_ref()
            .unwrap_or(&"".to_string())
            .starts_with("text/plain"))
        .map(|attachment| async {
            let content = attachment.download().await.unwrap();
            format!(
                "File: {}\n{}\n\n",
                attachment.filename,
                String::from_utf8_lossy(&content)
            )
        })
        .collect::<futures::future::JoinAll<_>>()
        .await
        .join("");

    if !formatted_content.is_empty() {
        let user_id = msg.author.id.to_string();

        let url = util::paste(&formatted_content)
            .await?;

        let formatted_message = format!("<@{}> has pasted:\n {}", user_id, url);

        msg.channel_id
            .say(&ctx.http, &formatted_message)
            .await?;
    }

    Ok(())
}


/// This will just paste the message content to a paste service.
pub async fn paste_message_handler(ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    let url = util::paste(&msg.content).await?;
    let formatted_response = format!("<@{}> has pasted:\n {}", msg.author.id, url);

    msg.channel_id
        .say(&ctx.http, &formatted_response)
        .await?;

    Ok(())
}