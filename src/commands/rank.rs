// rank.rs
use serenity::model::channel::Message;
use serenity::prelude::Context;
use sqlx::SqliteConnection;
use sqlx::Row;

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

    let message = format!(
        "You have {} XP and are level {}! You need {} XP to level up.",
        xp,
        level,
        xp_required - xp
    );

    msg.channel_id.say(&ctx.http, &message).await?;

    Ok(())
}
