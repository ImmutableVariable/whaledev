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

### Running

- Debug: `cargo run`
- Release: `cargo run --release`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
