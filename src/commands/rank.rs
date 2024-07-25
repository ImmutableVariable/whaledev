// rank.rs
use serenity::model::channel::Message;
use serenity::prelude::Context;
use sqlx::SqliteConnection;
use sqlx::Row;

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
    let user_id = msg.author.id.to_string();
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
    let xp_required = xp_constant * (level * level);

    // 50/100 = 0.5
    let progress = xp as f32 / xp_required as f32;
    let progress_bar = generate_progress_bar(progress, 10);
    
    let message = format!(
        "You are level {} with {} XP\n{}/{} [{}]",
        level, xp, xp, xp_required, progress_bar
    );

    msg.channel_id.say(&ctx.http, &message).await?;

    Ok(())
}
