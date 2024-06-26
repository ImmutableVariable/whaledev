const { Client, GatewayIntentBits } = require('discord.js');
const log = require('./utils/logger.js');  
const fs = require('fs');

let lastPasteTime = Date.now();

async function paste(message) {
    // paste.com only allows one request every second. This isn't an ideal solution, but I don't really care.
    if (Date.now() - lastPasteTime < 1000) {
        await new Promise(r => setTimeout(r, 1000));
    }

    let response = await fetch("https://dpaste.com/api/", {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
            "User-Agent": "Whaledev/1.0.0 (+https://github.com/ImmutableVariable/whaledev)"
        },
        body: "content=" + encodeURIComponent(message),
    });
    lastPasteTime = Date.now();

    return response.headers.get("location");
}

require('dotenv').config();

const client = new Client({ intents: [
    GatewayIntentBits.Guilds, 
    GatewayIntentBits.GuildMessages, 
    GatewayIntentBits.MessageContent
] });

client.textCommands = new Map();
client.cooldown = new Map();

// Text Commands Handler
for (const file of fs.readdirSync('./src/commands')) { 
    if (file.endsWith('.js')) {
        const command = require(`./commands/${file}`);
        if (!command.name || !command.execute) throw new Error(`[ERROR] Invalid command file structure: ${file}`); 

        log.debug(`[Commands] Loaded text command ${command.name}`);
        client.textCommands.set(command.name, command);
    }
}

// Events Handler
for (const file of fs.readdirSync('./src/events')) { 
    if (file.endsWith('.js')) {
        const event = require(`./events/${file}`);
        if (!event.name || !event.execute) throw new Error(`[ERROR] Invalid event file structure: ${file}`);

        log.debug(`[Events] Loaded event ${event.name} ${event.once ? '(once)' : ''}`);
        if (event.once) { 
            client.once(event.name, (...args) => event.execute(...args));
        } else {
            client.on(event.name, (...args) => event.execute(...args));
        }
    }
}

// message pasting
client.on("messageCreate", async message => {
    if (message.author.bot) return;

    let amalgamation = "";
    let reply = true;

    // process attached text files
    let attachments = message.attachments;
    for (let attachment of attachments) {
        // Discord url to download the attachment from
        let url = attachment[1].url;
        // contentType is kind of iffy if we're looking specifically for text files. For example, .rs files are lists as
        // a UTF-8 encoded executable (or something like that idk). Checking if it's "text" OR "utf-8" seems to work fine,
        // but some unpredicted file types that we don't want may slip through.
        let contentType = attachment[1].contentType;
        if (contentType.includes("text") || contentType.includes("utf-8")) {
            // get the file contents
            await fetch(url)
                .then(response => response.text())
                .then(async (data) => {
                    let pasteUrl = await paste(data);

                    if (pasteUrl !== "") {
                        amalgamation += `${attachment[1].name}: ${pasteUrl}\n`;
                    }
                    else {
                        message.channel.send(`Failed to create paste for file ${attachment[1].name}`);
                    }
                });
        }
    }

    // process message content
    if (message.cleanContent.length >= 1000) {
        let pasteUrl = await paste(message.cleanContent);
        amalgamation += `${message.author} posted: ${pasteUrl}`;
        reply = false;
        message.delete();
    }

    if (amalgamation !== "") {
        if (reply) {
            await message.reply(amalgamation);
        }
        else {
            await message.channel.send(amalgamation);
        }
    }
});

client.login(process.env.DISCORDTOKEN);