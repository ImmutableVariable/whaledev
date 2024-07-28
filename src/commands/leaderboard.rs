// leaderboard.rs
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
    // select the top 10 users by level and xp
    let rows = sqlx::query("SELECT * FROM users ORDER BY level DESC, xp DESC LIMIT 10")
        .fetch_all(db_conn)
        .await?;

    let mut fields = vec![];

    for (i, row) in rows.iter().enumerate() {
        let user_id = row.get::<String, _>("id");
        let user = ctx.http.get_user(user_id.parse::<u64>()?.into()).await?;
        let display = &user.global_name.unwrap_or("Unknown".to_string());
        let username = user.name;

        let xp = row.get::<i32, _>("xp");
        let level = row.get::<i32, _>("level");

        let xp_constant = std::env::var("XP_CONSTANT")?.parse::<i32>()?;
        let xp_required = util::xp_required(level, xp_constant);

        let progress = xp as f32 / xp_required as f32;
        let progress_bar = format!("({}) {} ({})", xp, generate_progress_bar(progress, 5), xp_required);

        fields.push((
            format!("{} | {} ({}) ", i + 1, display, username),
            format!("Level: {}\nXP: {}\nProgress: {}", level, xp, progress_bar),
            false,
        ));
    }

    if fields.is_empty() {
        fields.push(("No users found".to_string(), "No users found".to_string(), false));
    }

    let embed = builder::CreateEmbed::default()
        .title("Leaderboard")
        .description("Here are the top 10 users!")
        .fields(fields)
        .color(0xFFB6C1);

    let message = CreateMessage::new().embed(embed);

    msg.channel_id.send_message(&ctx.http, message).await?;

    Ok(())
}
