use dotenvy::dotenv;
use serenity::all::{ActivityData, ChannelId, CreateMessage, Member, OnlineStatus, Ready};
use serenity::model::channel::Message;
use serenity::{async_trait, builder};
use serenity::{futures, prelude::*};
use sqlx::sqlite::SqliteConnection;
use sqlx::{Connection, Row};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use util::should_paste_message;
mod commands;
pub mod util;

struct Handler {
    db_pool: Arc<Mutex<SqliteConnection>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == "302050872383242240".parse::<u64>().unwrap() && msg.embeds.is_empty(){
            if let Some(title) = &msg.embeds[0].title {
                if title != "DISBOARD: The Public Server List" {
                    return;
                }
            }

            if let Some(description) = &msg.embeds[0].description {
                if !description.contains("Bump done") {
                    return;
                }
            }

            msg.channel_id
                .say(&ctx.http, "Bump done, I will remind you in 2hr!")
                .await
                .expect("Error sending message, first bump done");

            let reminder_channel_id = std::env::var("REMINDER_CHANNEL_ID")
                .unwrap()
                .parse::<u64>()
                .unwrap();

            // wait 2 hours before sending the reminder
            tokio::time::sleep(tokio::time::Duration::from_secs(7200)).await;

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
                .expect("Error sending message");

            return;
        }

        if msg.author.bot {
            return;
        }

        // check if the message contains attachments, if it does, upload the attachments to a paste service
        // this must be before message length check, otherwise, the long message would be deleted and the attachments would be lost
        if !msg.attachments.is_empty() {
            let formatted_content = msg
                .attachments
                .iter()
                .filter(|&attachment| attachment
                    .content_type
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .starts_with("text/plain"))
                .map(|attachment| async {
                    let content = attachment.download().await.unwrap();
                    format!(
                        "File: {}\n{}\n\n",
                        attachment.filename,
                        String::from_utf8_lossy(&content)
                    )
                })
                .collect::<futures::future::JoinAll<_>>()
                .await
                .join("");

            if !formatted_content.is_empty() {
                let user_id = msg.author.id.to_string();

                let url = util::paste(&formatted_content)
                    .await
                    .expect("Error creating paste URL");
                let formatted_message = format!("<@{}> has pasted:\n {}", user_id, url);

                msg.channel_id
                    .say(&ctx.http, &formatted_message)
                    .await
                    .expect("Error sending message");
            }
        }

        if should_paste_message(msg.content.len()) {
            let url = util::paste(&msg.content)
                .await
                .expect("Error creating paste URL");
            let formatted_response = format!("<@{}> has pasted:\n {}", msg.author.id, url);
            msg.channel_id
                .say(&ctx.http, &formatted_response)
                .await
                .expect("Error sending message");
            msg.delete(&ctx.http).await.expect("Error deleting message");
            return;
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
            let user_id = msg.author.id.to_string();

            let row = sqlx::query("SELECT * FROM users WHERE id = ?")
                .bind(&user_id)
                .fetch_optional(&mut *conn)
                .await
                .expect("Error fetching user");

            if let Some(row) = row {
                let xp_constant = std::env::var("XP_CONSTANT")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let level: i32 = row.get("level");
                let xp: i32 = row.get("xp");
                let xp_required = xp_constant * (level * level);

                if xp >= xp_required {
                    sqlx::query("UPDATE users SET level = level + 1, xp = 0 WHERE id = ?")
                        .bind(&user_id)
                        .execute(&mut *conn)
                        .await
                        .expect("Error updating user");
                    msg.channel_id
                        .say(
                            &ctx.http,
                            format!(
                                "<@{}> has leveled up to level {}!",
                                msg.author.id, level + 1
                            ),
                        )
                        .await
                        .expect("Error sending message");
                } else {
                    sqlx::query("UPDATE users SET xp = xp + 1 WHERE id = ?")
                        .bind(&user_id)
                        .execute(&mut *conn)
                        .await
                        .expect("Error updating user");
                }
            } else {
                sqlx::query("INSERT INTO users (id, xp, level) VALUES (?, ?, ?)")
                    .bind(&user_id)
                    .bind(1)
                    .bind(0)
                    .execute(&mut *conn)
                    .await
                    .expect("Error inserting user");
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

#[tokio::main]
async fn main() {
    dotenv().ok(); // expects a .env file in the root directory

    let db_url = "sqlite://./data/database.db";
    
    // Ensure the directory for the database file exists
    std::fs::create_dir_all("./data").expect("Failed to create database directory");

    let mut conn = SqliteConnection::connect(db_url).await.unwrap();
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            xp INTEGER NOT NULL DEFAULT 0,
            level INTEGER NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(&mut conn)
    .await
    .unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        db_pool: Arc::new(Mutex::new(conn)),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error building client");

    client.start().await.expect("Error starting client");
}