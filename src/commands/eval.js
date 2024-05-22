const { exec } = require('child_process');

module.exports = {
    name: "eval",
    description: "Allows the bot owner to run commands for the server, it is blocked to all other users duhhhh",
    private: true,
    execute(message, args) {

        // check the server id to make sure it is the correct server, this command should only be available to the bot owner server
        if (message.guild.id !== process.env.GUILDID) {
            return;
        }

        if (message.author.id !== process.env.OWNERID) {
            message.channel.send("You are not the bot owner, you cannot run this command silly.");
            return;
        }

        if (!args.length) {
            message.channel.send("You need to provide a command to run silly. It must be a valid bash command.");
            return;
        }

        exec(args.join(" "), (error, stdout, stderr) => {
            if (error) {
                message.channel.send(`Error: ${error.message}`);
                return;
            }

            if (stderr) {
                message.channel.send(`Error: ${stderr}`);
                return;
            }

            message.channel.send(`Output: ${stdout}`);
        });
    }
}