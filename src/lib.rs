//! This library is an abstraction build on top of [`discord_rich_presence`]
//! crate that allows you to use it in a more declarative way.
//! 
//! When created it starts an infinite loop on a new thread that
//! will constantly try to reconnect the client if it's
//! enabled.
//! 
//! For more information, please also read the original library documentation: [`discord_rich_presence`]
//! 
//! [`discord_rich_presence`]: https://docs.rs/discord-rich-presence/
//! 
//! # Example
//! ```
//! use declarative_discord_rich_presence::DeclarativeDiscordIpcClient;
//! use declarative_discord_rich_presence::activity::Activity;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>>{
//! 
//!     let mut client = DeclarativeDiscordIpcClient::new("<some client id>");
//! 
//!     client.enable();
//!
//!     // It takes time for it to connect, so you can't set activity right away. Definetely a downside of this library for now. 
//!
//!     std::thread::sleep(std::time::Duration::from_secs(5));
//! 
//!     client.set_activity(Activity::new()
//!         .state("Hello world!")
//!         .details("Hello world!")
//!     )?;
//!
//!     std::thread::sleep(std::time::Duration::from_secs(10));
//! 
//!     client.clear_activity()?;
//!
//!     client.disable();
//!
//!     Ok(())
//! }
//! ```
#![deny(missing_docs)]

mod declarative_discord_ipc;
pub use declarative_discord_ipc::*;
pub use discord_rich_presence::*;
