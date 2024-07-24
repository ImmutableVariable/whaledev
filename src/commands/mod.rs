use serenity::model::channel::Message;
use serenity::prelude::Context;
mod eval;
mod paste;
mod ping;

pub async fn handler(ctx: Context, msg: &Message, command: &str, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "ping" => {
            ping::execute(ctx, &msg, args).await?;
        }
        "eval" => {
            eval::execute(ctx, &msg, args).await?;
        }
        "paste" => {
            paste::execute(ctx, &msg, args).await?;
        }
        _ => {
            // say nothing...
        }
    }

    Ok(())
}
