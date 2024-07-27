use serenity::all::{Context, Message};
use sqlx::sqlite::SqliteConnection;
use sqlx::Row;

use crate::util::{self, time_elapsed_from_string};

/// A handler to handle the xp system and generation of xp. It also handles user creation in the database.
pub async fn handler(ctx: &Context, msg: &Message, conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = msg.author.id.to_string();

    let row = sqlx::query("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(&mut *conn)
        .await?;

    if let Some(row) = row {
        let xp_constant = std::env::var("XP_CONSTANT")?.parse::<i32>()?;
        let xp_timeout = std::env::var("XP_TIMEOUT")?.parse::<i64>()?;
        let level: i32 = row.get("level");
        let xp: i32 = row.get::<i32, _>("xp") + 1; // add 1 xp to the user to represent the current message being sent so i dont end up with 5/5 xp error
        let xp_required = util::xp_required(level, xp_constant);
    
        let last_message_at: String = row.get("updated_at");
        let duration = time_elapsed_from_string(&last_message_at).unwrap();
        if duration.num_seconds() < xp_timeout {
            return Ok(());
        }

        if xp >= xp_required {
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
        
            sqlx::query("UPDATE users SET level = level + 1, xp = 0, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(&user_id)
                .execute(&mut *conn)
                .await
                .expect("Error updating user");
        } else {
            sqlx::query("UPDATE users SET xp = xp + 1, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
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

    Ok(())
}