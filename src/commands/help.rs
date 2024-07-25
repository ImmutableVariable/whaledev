// help.rs
use serenity::all::CreateMessage;
use serenity::builder;
use serenity::model::channel::Message;
use serenity::prelude::Context;

const COMMANDS_AND_DESCRIPTIONS: [(&str, &str); 3] = [
    ("ping", "Responds with 'Pong!'"),
    ("paste", "Copies a code block to the DPaste"),
    ("help", "Displays this message"),
];

pub async fn execute(ctx: Context, msg: &Message, _args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let prefix = std::env::var("PREFIX")?;
    let usage_message = format!("Usage: **{}**<command> <arguments*>", prefix);

    let embed_fields = COMMANDS_AND_DESCRIPTIONS.iter().map(|(command, description)| {
        (command.to_string(), description.to_string(), false)
    }).collect::<Vec<_>>();

    let embed = builder::CreateEmbed::default()
        .title("Here is some help!")
        .description(usage_message)
        .fields(embed_fields)
        .color(0xFFB6C1);

    let message = CreateMessage::new()
        .embed(embed);

    msg.channel_id
        .send_message(&ctx.http, message)
        .await?;
    Ok(())
}