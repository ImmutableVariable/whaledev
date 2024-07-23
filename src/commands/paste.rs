// paste.rs
use serenity::model::channel::Message;
use serenity::prelude::Context;
use crate::util;

pub async fn execute(ctx: Context, msg: &Message, args: Vec<&str>) {
   let content = args.join(" ");
   let mut url = util::paste(&content).await.unwrap();
   let user_id = msg.author.id.to_string();
   // <AUTHOR> has pasted: 
   // <URL>
   url = format!("<@{}> has pasted:\n {}", user_id, url);
   msg.channel_id.say(&ctx.http, &url).await.expect("Error sending message");
}