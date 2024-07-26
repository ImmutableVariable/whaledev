use std::env;
use std::sync::Arc;

use serenity::all::{ActivityData, ChannelId, CreateMessage, Member, OnlineStatus, Ready};
use serenity::model::channel::Message;
use serenity::{async_trait, builder};
use serenity::prelude::*;
use sqlx::sqlite::SqliteConnection;
use crate::{commands, util};
use util::should_paste_message;

mod xp;
mod paste;
mod bump;

/// A struct that implements the EventHandler trait, which is the main bot event handler.
pub struct Handler {
    pub db_pool: Arc<Mutex<SqliteConnection>>,
}


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == "302050872383242240".parse::<u64>().unwrap() && msg.embeds.len() > 0 {
            return bump::bump_message_handler(ctx, msg).await.expect("Error bumping message");
        }
    
        if msg.author.bot {
            return;
        }
    
        // check if the message contains attachments, if it does, upload the attachments to a paste service
        // this must be before message length check, otherwise, the long message would be deleted and the attachments would be lost
        if !msg.attachments.is_empty() {
            return paste::paste_file_handler(&ctx, &msg).await.expect("Error pasting file");
        }
    
        if should_paste_message(msg.content.len()) {
            return paste::paste_message_handler(ctx, msg).await.expect("Error pasting message");
        }
    
        let prefix = env::var("PREFIX").unwrap();
        if msg.content.starts_with(&prefix) {
            let mut conn = self.db_pool.lock().await;
            let content = msg.content.trim_start_matches(&prefix);
            let mut args = content.split_whitespace();
            let command = args.next().unwrap_or(""); // the first word will be the command name
            let args = args.collect::<Vec<&str>>(); // the rest of the words will be the arguments
    
            match commands::handler(ctx, &msg, command, args, &mut conn).await {
                Ok(_) => {}
                Err(why) => {
                    println!("Error handling command: {:?}", why);
                }
            }
    
            return;
        }
    
        if msg.content.len() >= 5 {
            let mut conn = self.db_pool.lock().await;
            match xp::handler(&ctx, &msg, &mut conn).await {
                Ok(_) => {}
                Err(why) => {
                    println!("Error handling xp: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let prefix = std::env::var("PREFIX").unwrap();
        let activity = ActivityData::watching(format!("for {}help", prefix));
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status);
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let channel_id = std::env::var("WELCOME_CHANNEL_ID")
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let guild_member_count = new_member
            .guild_id
            .members(&ctx.http, None, None)
            .await
            .expect("Error getting member count")
            .len();

        let new_member_id = new_member.user.id;

        let embed = builder::CreateEmbed::default()
            .title(format!(
                "Welcome, {}, to the server! 🎉",
                new_member.user.name
            ))
            .description(format!(
                "You are the {} member!",
                util::formatted_number(guild_member_count as u64)
            ))
            .thumbnail(new_member.user.face())
            .color(0xFFB6C1);

        let message = CreateMessage::new()
            .content(format!(
                "||<@&{}> <@{}>||",
                std::env::var("REMINDER_ROLE_ID").unwrap(),
                new_member_id
            ))
            .embed(embed);
        let channel = ChannelId::new(channel_id);

        channel
            .send_message(&ctx.http, message)
            .await
            .expect("Error sending message");
    }
}