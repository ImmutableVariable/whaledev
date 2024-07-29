use dotenvy::dotenv;
use serenity::prelude::*;
use std::sync::Arc;

pub mod commands;
pub mod util;
pub mod events;

#[tokio::main]
async fn main() {
    dotenv().ok(); // expects a .env file in the root directory

    let db_path: &str = "sqlite://./data/database.db";

    let pool = sqlx::sqlite::SqlitePool::connect(db_path)
        .await
        .expect("Error connecting to the database");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            xp INTEGER NOT NULL DEFAULT 0,
            level INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(&mut *pool.acquire().await.unwrap())
    .await
    .unwrap();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected a discord token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = events::Handler {
        db_pool: Arc::new(pool),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error building client");

    match client.start().await {
        Ok(_) => {}
        Err(why) => {
            println!("Client error: {:?}", why);
        }
    }
}