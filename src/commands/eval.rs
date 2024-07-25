use crate::util::execute_console_command;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::env;

pub async fn execute(
    ctx: Context,
    msg: &Message,
    args: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Cache environment variables to avoid repeated calls
    let owner_id = env::var("OWNER_ID")?;
    let owner_guild_id = env::var("OWNER_GUILD_ID")?;

    if msg.guild_id.is_none() {
        msg.channel_id.say(&ctx.http, "Guild ID not found.").await?;
        return Ok(());
    }

    if owner_id != msg.author.id.to_string()
        || owner_guild_id
            != msg
                .guild_id
                .ok_or("Guild ID not found... somehow?")?
                .to_string()
    {
        msg.channel_id
            .say(&ctx.http, "You are not the owner of this bot.")
            .await?;
        return Ok(());
    }

    if args.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please provide a code to evaluate.")
            .await?;
        return Ok(());
    }

    let code = args.join(" ");
    let output = execute_console_command(&code)?;
    msg.channel_id.say(&ctx.http, output).await?;
    Ok(())
}
