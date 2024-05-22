const { PermissionsBitField } = require("discord.js");

module.exports = {
    name: "pingchallenges",
    description: "Ping the challenges role to notify them of a new challenge. This command is only available to CHALLENGEPOSTERROLE",
    usage: "<prefix>pingchallenges",
    private: false,
    execute(message) {
        const { CHALLENGEPOSTERROLE, CHALLENGEPINGROLE, CHALLENGEANNOUNCEMENTCHANNEL } = process.env;

        const hasChallengePosterRole = message.member.roles.cache.has(CHALLENGEPOSTERROLE);
        const hasModeratorPermission = message.member.permissions.has([PermissionsBitField.Flags.KickMembers, PermissionsBitField.Flags.BanMembers]);

        if (!hasChallengePosterRole && !hasModeratorPermission) {
            message.channel.send("You do not have permission to use this command.");
            return;
        }

        const announcementChannel = message.client.channels.cache.get(CHALLENGEANNOUNCEMENTCHANNEL);
        announcementChannel.send(`<@&${CHALLENGEPINGROLE}>`);

        // if the invoking message is deletable,
        // delete it after 5 seconds to give the mod a chance to see the message
        setTimeout(() => {
            if (message.deletable) {
                message.delete();
            }
        }, 3000);

    }
}