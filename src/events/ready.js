const logger = require('../utils/logger.js');
const { ActivityType } = require('discord.js');

module.exports = {
    name: 'ready',
    once: true,
    async execute(client) {
        logger.info(`Logged in as ${client.user.tag}`);

        const presence = {
            activities: [{
                name: `for ${process.env.PREFIX}help`,
                type: ActivityType.Watching
            }],
            status: 'online',
        }
        
        client.user.setPresence(presence);

        setInterval(() => {
            client.user.setPresence(presence);
        }, 7_200_000); // 2 hours
    }
}