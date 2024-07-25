//ping.rs
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn execute(ctx: Context, msg: &Message, _args: Vec<&str>) -> Result<(), serenity::Error> {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}
