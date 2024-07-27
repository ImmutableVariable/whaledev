use serenity::model::channel::Message;
use serenity::prelude::Context;
use sqlx::SqliteConnection;
mod help;
mod paste;
mod ping;
mod rank;

pub async fn handler(
    ctx: Context,
    msg: &Message,
    command: &str,
    args: Vec<&str>,
    db_conn: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "ping" => {
            ping::execute(ctx, msg, args).await?;
        }
        "paste" => {
            paste::execute(ctx, msg, args).await?;
        }
        "help" => {
            help::execute(ctx, msg, args).await?;
        }
        "rank" => {
            rank::execute(ctx, msg, args, db_conn).await?;
        }
        _ => {
            // say nothing...
        }
    }

    Ok(())
}
