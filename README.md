## Setting up giphybot

Create a configuration toml file with the app's configuration.  It should look like
this:

    [general]
    port = "9999"

    [giphy]
    api_key = "GIPHY_API_KEY"

    [braid]
    name = "nameofthisbotonbraid"
    api_url = "https://api.braid.chat/bots/message"
    token = "BOT_BRAID_TOKEN"

There is a public giphy api key to use for testing: `dc6zaTOxFJmzC`

Specify the file as the first command-line argument, e.g.

    $ giphybot conf.toml
