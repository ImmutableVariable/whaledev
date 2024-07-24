use dotenv::dotenv;
use serenity::all::{ChannelId, CreateMessage, Member, Ready};
use serenity::model::channel::Message;
use serenity::{futures, prelude::*};
use serenity::{async_trait, builder};
use std::env;
mod commands;
pub mod util;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content.len() > env::var("MAX_MESSAGE_LENGTH")
            .unwrap()
            .parse::<usize>()
            .unwrap()
        {
            let url = util::paste(&msg.content).await.unwrap();
            let user_id = msg.author.id.to_string();
            let url = format!("<@{}> has pasted:\n {}", user_id, url);
            msg.channel_id
                .say(&ctx.http, &url)
                .await
                .expect("Error sending message");
            msg.delete(&ctx.http).await.expect("Error deleting message");
            return;
        }

        // check if the message contains attachments, if it does, upload the attachments to a paste service
        if !msg.attachments.is_empty() {
            let formatted_content = msg.attachments.iter()
                .filter_map(|attachment| {
                    attachment.content_type.as_ref()?
                        .starts_with("text/plain")
                        .then(|| attachment)
                })
                .map(|attachment| async {
                    let content = attachment.download().await.unwrap();
                    format!("File: {}\n{}\n\n", attachment.filename, String::from_utf8_lossy(&content))
                })
                .collect::<futures::future::JoinAll<_>>()
                .await
                .join("");
        
            if !formatted_content.is_empty() {
                let user_id = msg.author.id.to_string();
                let url = util::paste(&formatted_content).await.unwrap();
                let formatted_message = format!("<@{}> has pasted:\n {}", user_id, url);
        
                msg.channel_id.say(&ctx.http, &formatted_message).await.expect("Error sending message");
                msg.delete(&ctx.http).await.expect("Error deleting message");
            }
        }


        let prefix = env::var("PREFIX").unwrap();
        if msg.content.starts_with(&prefix) {
            let content = msg.content.trim_start_matches(&prefix);
            let mut args = content.split_whitespace();
            let command = args.next().unwrap_or(""); // the first word will be the command name
            let args = args.collect::<Vec<&str>>(); // the rest of the words will be the arguments

            match commands::handler(ctx, &msg, command, args).await {
                Ok(_) => {}
                Err(why) => {
                    println!("Error handling command: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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
            .description(format!("You are the {} member!", util::formatted_number(guild_member_count as u64)))
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

#[tokio::main]
async fn main() {
    dotenv().ok(); // expects a .env file in the root directory

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error building client");

    client.start().await.expect("Error starting client");
}
