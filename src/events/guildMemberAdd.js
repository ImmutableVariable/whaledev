const { EmbedBuilder } = require("discord.js")

function formatMemberNumber(memberCount) {
    const lastDigit = memberCount % 10;
    const lastTwoDigits = memberCount % 100;
    const number = new Intl.NumberFormat().format(memberCount); // format to locale string
    
    if (lastTwoDigits >= 11 && lastTwoDigits <= 13) {
        return `${number}th`;
    } else if (lastDigit === 1) {
        return `${number}st`;
    } else if (lastDigit === 2) {
        return `${number}nd`;
    } else if (lastDigit === 3) {
        return `${number}rd`;
    } else {
        return `${number}th`;
    }
}

module.exports = {
    name: 'guildMemberAdd',
    async execute(member) {
        const welcomeChannel = member.guild.channels.cache.get(process.env.WELCOMECHANNELID) 

        if (!welcomeChannel) return;

        const memberName = member.user.username;
        const memberId = member.user.id;
        const memberCount = member.guild.memberCount;
        const prettyNumber = formatMemberNumber(memberCount);

        const embed = new EmbedBuilder()
            .setTitle(`Welcome, ${memberName}, to the server! 🎉`)
            .setDescription(`You are the ${prettyNumber} member!`)
            .setThumbnail(member.user.displayAvatarURL())
            .setColor(0xFFB6C1)

        welcomeChannel.send({content: `||<@&${process.env.REMINDERROLEID}> <@${memberId}>||`, embeds: [embed]});
    }
}