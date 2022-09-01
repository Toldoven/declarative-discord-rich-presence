use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
use declarative_discord_rich_presence::activity::Activity;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>>{

    let mut client = DeclarativeDiscordIpcClient::new("771124766517755954");

    client.enable();

    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("Setting activity...");

    client.set_activity(Activity::new()
        .state("Hello world!")
        .details("Hello world!")
    )?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("Clearing activity...");

    client.clear_activity()?;

    std::thread::sleep(std::time::Duration::from_secs(15));

    println!("Setting activity...");

    client.set_activity(Activity::new()
        .state("Hello world!")
        .details("Hello world!")
    )?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    client.disable();

    Ok(())
}
