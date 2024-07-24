// paste.rs
use crate::util;
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn execute(ctx: Context, msg: &Message, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let content = args.join(" ");
    let mut url = util::paste(&content).await?;
    let user_id = msg.author.id.to_string();
    url = format!("<@{}> has pasted:\n {}", user_id, url);
    msg.channel_id
        .say(&ctx.http, &url)
        .await?;

    Ok(())
}