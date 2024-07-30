use serenity::all::{ChannelId, Context, Message};


/// This will remind you if the bump has already been detected
pub async fn bump_message_handler(ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(title) = &msg.embeds[0].title {
        if title != "DISBOARD: The Public Server List" {
            return Ok(());
        }
    }

    if let Some(description) = &msg.embeds[0].description {
        if !description.contains("Bump done") {
            return Ok(());
        }
    }

    msg.channel_id
        .say(&ctx.http, "Bump done, I will remind you in 2hr!")
        .await?;

    tokio::spawn(async move {
        // wait 2 hours before sending the reminder
        tokio::time::sleep(tokio::time::Duration::from_secs(7200)).await;
        let reminder_channel_id = std::env::var("REMINDER_CHANNEL_ID")
            .expect("REMINDER_CHANNEL_ID must be set")
            .parse::<u64>()
            .expect("REMINDER_CHANNEL_ID must be a valid u64");
        
        let reminder_channel = ChannelId::new(reminder_channel_id);
        reminder_channel
            .say(
                &ctx.http,
                format!(
                    "<@&{}>, it's time to bump the server!",
                    std::env::var("REMINDER_ROLE_ID").unwrap()
                ),
            )
            .await
            .expect("Error sending reminder message");
    });

    Ok(())
}