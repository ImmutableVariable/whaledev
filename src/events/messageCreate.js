const logger = require('../utils/logger.js');
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
        if (!message.content.startsWith(prefix)) return;

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