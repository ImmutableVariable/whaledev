# Whaledev

Whaledev is a discord bot written using [Serenity](https://www.github.com/serenity-rs/serenity). It is only used for managing my personal server so it is really basic; however, it is open source so feel free to use it as a basis for a future project. It features bump reminders, a welcome message, and a few other commands.

## Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Discord

1. Create a new application on the [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a bot for the application in the "Bot" tab, IT MUST HAVE THE GATEWAY INTENT "Server Members Intent" AND "Message Content Intent" ENABLED
3. Reset the token for the bot, then copy it into the .env file as `DISCORD_TOKEN`
4. Add your bot in the "OAuth2" tab with the "bot" scope and the "Administrator" permission

### Environment Variables

1. Rename the `.env.example` file to `.env`
2. `DISCORD_TOKEN` - your bot's token (DANGER! DO NOT SHARE THIS. ANYONE CAN CONTROL YOUR BOT WITH THIS)
3. `OWNER_ID` - Discord ID (DANGER! USERS CAN RUN ARBITRARY CODE WITH THE "EVAL" COMMAND)
4. `OWNER_GUILD_ID` - ID of the server you want the bot to be able to use the "eval" command in (DANGER! USERS CAN RUN ARBITRARY CODE WITH THE "EVAL" COMMAND)
5. `WELCOME_CHANNEL_ID` - channel the welcome messages will be sent to.
6. `REMINDER_ROLE_ID` - ID of the role that will be mentioned in the bump reminder.
7. `REMINDER_CHANNEL_ID` - ID of the channel the bump reminder will be sent to.
8. `MAX_MESSAGE_LENGTH` - maximum length of a message that can be sent in the server. (Default is 1500)
9. `PREFIX` - prefix you want the bot to use. (Default is `!`)

### Running

- Debug: `cargo run`
- Release: `cargo run --release`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
