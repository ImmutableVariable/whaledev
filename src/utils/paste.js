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


module.exports = paste;