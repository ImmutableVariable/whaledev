module.exports = {
    name: "help",
    description: "prints the help message for all the commands",
    usage: "<prefix>pingchallenges",
    private: false,
    execute(message) {
        const commands = message.client.textCommands;

        const data = [];

        if (!commands) {
            return message.channel.send("There are no commands available.");
        }

        data.push("**Here's a list of all my commands:**");

        data.push("\n```"); // creates a code block for the commands to sit in
        commands.forEach((command) => {
            if (command.private) return;

            data.push(`Name: ${command.name}`);
            data.push(`Description: ${command.description}`);
            data.push(`Usage: ${command.usage}`);
            data.push(`\n`)
        });
        data.push("```");

        message.channel.send(data.join("\n"));
    }
}