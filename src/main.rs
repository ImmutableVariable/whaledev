use dotenvy::dotenv;
use serenity::prelude::*;
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod commands;
pub mod util;
pub mod events;

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
            level INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
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

    let handler = events::Handler {
        db_pool: Arc::new(Mutex::new(conn)),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error building client");

    client.start().await.expect("Error starting client");
}