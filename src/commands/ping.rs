//ping.rs
use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn execute(ctx: Context, msg: &Message, _args: Vec<&str>) {
    msg.channel_id.say(&ctx.http, "Pong!").await.expect("Error sending message");
}