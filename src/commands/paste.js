const paste = require("../utils/paste");

module.exports = {
    name: "paste",
    description: "Paste the message to dpaste.com and return the link.",
    usage: "<prefix>paste <message>",
    private: false,
    async execute(message, args) {
        const content = args.join(" ");

        if (!content) {
            return message.channel.send("You need to provide a message to paste.");
        }

        const url = await paste(content);

        if (!url) {
            return message.channel.send("There was an error while pasting the message.");
        }

        const authorId = message.author.id;
        message.channel.send(`<@${authorId}> your message has been pasted: ${url}`);
    }
}