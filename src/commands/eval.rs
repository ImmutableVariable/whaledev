//eval.rs
use std::env;

use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn execute(ctx: Context, msg: &Message, args: Vec<&str>) {
    // make sure it is env::var("OWNER_ID")
    let owner_id = env::var("OWNER_ID").expect("Expected an owner ID in the environment");
    let owner_guild_id = env::var("OWNER_GUILD_ID").expect("Expected an owner guild ID in the environment");

    if owner_id != msg.author.id.to_string() || owner_guild_id != msg.guild_id.unwrap().to_string() {
        msg.channel_id.say(&ctx.http, "You are not allowed to use this!").await.expect("Error sending message");
        return;
    }

    let code = args.join(" ");
    // determine the command to run based on the OS
    let command = if cfg!(windows) { "cmd" } else { "sh" };
    // execute the command
    let output = std::process::Command::new(command)
        .arg("/C")
        .arg(&code)
        .output()
        .expect("Failed to execute command");

    let output = format!("```bash\n{}```", String::from_utf8_lossy(&output.stdout));

    msg.channel_id.say(&ctx.http, &*output).await.expect("Error sending message");
}