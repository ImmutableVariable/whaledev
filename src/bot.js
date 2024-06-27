const { Client, GatewayIntentBits } = require('discord.js');
const log = require('./utils/logger.js');  
const fs = require('fs');

require('dotenv').config();

const client = new Client({ intents: [
    GatewayIntentBits.Guilds, 
    GatewayIntentBits.GuildMessages, 
    GatewayIntentBits.GuildMembers,
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

client.login(process.env.DISCORDTOKEN);