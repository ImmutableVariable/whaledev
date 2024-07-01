const logger = require('../utils/logger.js');
const paste = require('../utils/paste.js');

async function processMessage(message) {
    let amalgamation = "";
    let reply = true;

    let attachments = message.attachments.values();
    for (let attachment of attachments) {
        let url = attachment.url;
        let contentType = attachment.contentType;

        if (contentType && (contentType.includes("text") || contentType.includes("utf-8"))) {
            try {
                const data = await fetch(url).then(response => response.text());
                const pasteUrl = await paste(data);

                if (pasteUrl) {
                    amalgamation += `${attachment.name}: ${pasteUrl}\n`;
                } else {
                    await message.channel.send(`Failed to create paste for file ${attachment.name}`);
                }
            } catch (error) {
                logger.error(`Error processing attachment ${attachment.name}:`, error);
            }
        }
    }

    if (message.cleanContent.length >= process.env.MINMESSAGEPASTELENGTH) {
        const pasteUrl = await paste(message.cleanContent);
        if (pasteUrl) {
            amalgamation += `${message.author} posted: ${pasteUrl}`;
            reply = false;
            await message.delete();
        }
    }

    if (amalgamation) {
        if (reply) {
            await message.reply(amalgamation);
        } else {
            await message.channel.send(amalgamation);
        }
    }
}

module.exports = {
    name: 'messageCreate',
    async execute(message) {
        if (message.author.id === "302050872383242240" && message.embeds.length > 0) {
            if (message.embeds[0].title !== "DISBOARD: The Public Server List" || !(message.embeds[0].description.includes("Bump done"))) return;

            await message.channel.send(`Bump done, I will remind you in 2hr!`)

            setTimeout(() => {
                const reminderChannel = message.client.channels.cache.get(process.env.REMINDCHANNELID);
                reminderChannel.send(`<@&${process.env.REMINDERROLEID}>, it's time to bump the server!`);
            }, 7200000);

            return;
        }

        if (message.author.bot) return;


        const prefix = process.env.PREFIX;
        const prefixUpper = prefix.toUpperCase();

        if (!message.content.startsWith(prefix) && !message.content.startsWith(prefixUpper)) {
            await processMessage(message);
            return;
        }

        const args = message.content.trim().split(/ +/);
        const commandName = args.shift().toLowerCase();

        const command = message.client.textCommands.get(commandName.slice(prefix.length));

        if (!command) return;

        try {
            command.execute(message, args);
        } catch (error) {
            logger.error(error);
            message.reply('There was an error while executing this command!');
        }
    }
}