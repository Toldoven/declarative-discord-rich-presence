use std::{sync::{Arc, Mutex}, thread, time::Duration, error::Error};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};

#[doc(hidden)]
pub struct IpcClientWrapper {
    pub enabled: bool,
    pub activity: Option<Activity>,
    pub client: DiscordIpcClient,
}

impl IpcClientWrapper {
    fn new(client_id: &str) -> IpcClientWrapper {
        IpcClientWrapper {
            enabled: false,
            activity: None,
            client: DiscordIpcClient::new(client_id).unwrap()
        }
    }
}

/// Struct that implements all of the crates functionality
pub struct DeclarativeDiscordIpcClient (pub Arc<Mutex<IpcClientWrapper>>);

impl DeclarativeDiscordIpcClient {

    /// Creates a new `DeclarativeDiscordIpcClient`.
    /// 
    /// It also starts an infinite loop on a new thread that
    /// will constantly try to reconnect the client if it's
    /// enabled.
    ///
    /// # Examples
    /// ```
    /// let ipc_client = DeclarativeDiscordIpcClient::new("<some client id>")?;
    /// ```
    pub fn new(client_id: &str) -> DeclarativeDiscordIpcClient {

        let client = DeclarativeDiscordIpcClient(
            Arc::new(
                Mutex::new(
                    IpcClientWrapper::new(client_id)
                )   
            )
        );

        client.start_loop();

        client
    }

    /// Enable Discord IPC client. When enabled it always try to automatically reconnect when the client is not connected.
    pub fn enable(&self) {

        let mut lock = self.0.lock().unwrap();

        if lock.enabled { return }

        lock.enabled = true;
    }

    /// Disable Discord IPC client.
    pub fn disable(&self) {

        let mut lock = self.0.lock().unwrap();

        if !lock.enabled { return }

        lock.enabled = false;
    }

    fn start_loop(&self) {

        let clone = Arc::clone(&self.0);

        thread::spawn(move || { loop {

            thread::sleep(Duration::from_secs(1));

            let mut lock = clone.lock().unwrap();
            // let activity = &lock.activity.unwrap().clone();

            if !lock.enabled {
                if lock.client.connected { lock.client.close().ok(); }
                continue;
            };

            if lock.client.connected { continue };

            let connect = lock.client.connect();

            let activity = lock.activity.clone();

            if connect.is_ok() && activity.is_some() {
                lock.client.set_activity(activity.unwrap()).ok();
            }

        }});

    }


    /// Sets a Discord activity.
    /// 
    /// Returns `Ok` If client isn't supposed to be connected.
    /// You can safely call this function even if the client is
    /// disabled or not connected.
    ///
    /// # Errors
    /// If the `set_activity` function failed, it tries to
    /// reconnect, and only returns `Err` if it failed to
    /// send the payload after reconnecting.
    pub fn set_activity(&self, activity_payload: Activity) -> Result<(), Box<dyn Error>> {

        let mut lock = self.0.lock().unwrap();
        
        lock.activity = Some(activity_payload.clone());

        if !lock.client.connected { return Ok(()) }

        if lock.client.set_activity(activity_payload.clone()).is_err() { lock.client.reconnect()? }

        lock.client.set_activity(activity_payload)?;

        Ok(())

    }

    /// Clears a Discord activity.
    /// 
    /// Works exactly like `set_activity`
    /// 
    /// [`set_activity`]: #method.set_activity
    pub fn clear_activity(&self) -> Result<(), Box<dyn Error>> {

        let mut lock = self.0.lock().unwrap();

        lock.activity = None;

        if !lock.client.connected { return Ok(()) }

        if lock.client.clear_activity().is_err() { lock.client.reconnect()? }

        lock.client.clear_activity()?;

        Ok(())

    }
}