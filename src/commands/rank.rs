// rank.rs
use serenity::all::CreateMessage;
use serenity::builder;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use sqlx::SqliteConnection;
use sqlx::Row;

use crate::util;

fn generate_progress_bar(progress: f32, length: usize) -> String {
    (0..length)
        .map(|i| if i as f32 / length as f32 <= progress { "█" } else { "░" })
        .collect()
}

pub async fn execute(
    ctx: Context,
    msg: &Message,
    _args: Vec<&str>,
    db_conn: &mut SqliteConnection
) -> Result<(), Box<dyn std::error::Error>> {
    let user = msg.mentions.get(0).unwrap_or(&msg.author);
    let username = &user.name;
    let user_id = user.id.to_string();

    let row = sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(db_conn)
        .await?;

    let xp = match &row {
        Some(row) => row.get::<i32, _>("xp"),
        None => 0,
    };
    let level = match &row {
        Some(row) => row.get::<i32, _>("level"),
        None => 0,
    };

    let xp_constant = std::env::var("XP_CONSTANT")?.parse::<i32>()?;
    let xp_required = util::xp_required(level, xp_constant);

    // 50/100 = 0.5
    let progress = xp as f32 / xp_required as f32;
    let progress_bar = format!("({}) {} ({})", 0, generate_progress_bar(progress, 10), xp_required);
    
    let embed = builder::CreateEmbed::default()
        .title(&format!("{}'s Rank!", username))
        .description(&format!("Here is {}'s rank!", username))
        .fields(vec![
            ("Level", level.to_string(), false),
            ("XP", xp.to_string(), false),
            ("Progress", progress_bar, false),
        ])
        .color(0xFFB6C1);

    let message = CreateMessage::new().embed(embed);

    msg.channel_id.send_message(&ctx.http, message).await?;

    Ok(())
}
