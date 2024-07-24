// help.rs
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn execute(ctx: Context, msg: &Message, _args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let prefix = std::env::var("PREFIX")?;
    let usage_message = format!("Usage: {}<command>", prefix);
    let help_messages = vec![
        "**Here's a list of all my commands:**",
        "```",
        &usage_message,
        "Available commands:",
        "  - ping: Responds with 'Pong!'",
        "  - paste: Copies a code block to the DPaste",
        "```",
    ];

    msg.channel_id
        .say(&ctx.http, help_messages.join("\n"))
        .await?;

    Ok(())
}