## Setting up giphybot

Giphybot is written in Rust, so you'll probably want to use [rustup][] to install the rust toolchain if you don't already have it set up.
Alternatively, you can use a [pre-built binary](https://github.com/braidchat/giphybot/releases).

Create a configuration toml file with the app's configuration.  It should look like this:

    [general]
    port = "9999"

    [giphy]
    api_key = "GIPHY_API_KEY"

    [braid]
    name = "nameofthisbotonbraid"
    api_url = "https://api.braid.chat/bots/message"
    app_id = "BOT_BRAID_ID"
    token = "BOT_BRAID_TOKEN"

There is a public giphy api key to use for testing: `dc6zaTOxFJmzC`

Specify the file as the first command-line argument, e.g.

    $ giphybot conf.toml

You can run the bot for testing by doing `cargo run conf.toml`.

To deploy, build with `cargo build --release` then upload the generated binary from `target/release/giphybot`.
If building on a different architecture than you'll be deploying to, look into [rustup cross-compilation][crosscomp].

  [rustup]: https://www.rustup.rs/
  [crosscomp]: https://github.com/rust-lang-nursery/rustup.rs#cross-compilation
