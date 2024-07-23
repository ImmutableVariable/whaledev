use std::env;
use dotenv::dotenv;
use serenity::all::{ChannelId, CreateMessage, Member, Ready};
use serenity::{async_trait, builder};
use serenity::model::channel::Message;
use serenity::prelude::*;
pub mod util;
mod commands;

const PREFIX: &str = "!";
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content.len() > env::var("MAX_MESSAGE_LENGTH").unwrap().parse::<usize>().unwrap() {
            let url = util::paste(&msg.content).await.unwrap();
            let user_id = msg.author.id.to_string();
            let url = format!("<@{}> has pasted:\n {}", user_id, url);
            msg.channel_id.say(&ctx.http, &url).await.expect("Error sending message");
            msg.delete(&ctx.http).await.expect("Error deleting message");
            return;
        }
        
        // check if message starts with the prefix (in this case, "!")
        if msg.content.starts_with(PREFIX) {
            let content = msg.content.trim_start_matches(PREFIX);
            let mut args = content.split_whitespace();
            let command = args.next().unwrap_or(""); // the first word will be the command name
            let args = args.collect::<Vec<&str>>(); // the rest of the words will be the arguments

            commands::handler(ctx, &msg, command, args).await;
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let channel_id = std::env::var("WELCOME_CHANNEL_ID").unwrap().parse::<u64>().unwrap();
        let new_member_id = new_member.user.id;
    
        let embed = builder::CreateEmbed::default()
            .title(format!("Welcome, {}, to the server! 🎉", new_member.user.name))
            .description(format!("Welcome to the server!"))
            .thumbnail(new_member.user.face())
            .color(0xFFB6C1);
        let message = CreateMessage::new().content(format!("||<@&{}> <@{}>||", std::env::var("REMINDER_ROLE_ID").unwrap(), new_member_id)).embed(embed);
        let channel_id = ChannelId::new(channel_id);
        channel_id.send_message(&ctx.http, message).await.expect("Error sending message");
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

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}