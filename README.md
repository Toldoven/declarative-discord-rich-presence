# Declarative Discord Rich Presence

This library is an abstraction build on top of [discord-rich-presence](https://crates.io/crates/discord-rich-presence/) crate that allows you to use it in a more declarative way.

When created it starts an infinite loop on a new thread that will constantly try to reconnect the client if it's enabled.

For more information, please also read the original library documentation: [discord-rich-presence](https://docs.rs/discord-rich-presence/)

## Example
```rust
use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use declarative_discord_rich_presence::activity::Activity;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut client = DeclarativeDiscordIpcClient::new("771124766517755954");

    client.enable();

    client.set_activity(Activity::new()
        .state("Hello world!")
        .details("Hello world!")
    )?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    client.clear_activity()?;

    client.disable();

    Ok(())
}
```
