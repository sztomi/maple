use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use log;

mod client;
mod appstate;
mod ui;
mod logging;
mod config;

use client::{Client, ClientEvent};
use appstate::{App, AppState};
use ui::run_ui;
use logging::setup_logging;
use config::ensure_config_file;



fn main() -> Result<()> {
  setup_logging();
  log::info!("Maple for Plex starting");

  if let Ok(cfg_file) = ensure_config_file() {
    log::info!("Using config file: {}", cfg_file.display());
  }

  // this architecture is largely based on https://keliris.dev/improving-spotify-tui/ (<3)
  let app = Arc::new(Mutex::new(App::new(AppState::LoggedOut)));
  let cloned_app = Arc::clone(&app);
  let (tx, rx) = channel::<ClientEvent>();

  thread::spawn(move || {
    let mut client = Client::new(&app).unwrap();
    start_client(rx, &mut client);
  });

  run_ui(&cloned_app, tx)
}

#[tokio::main]
async fn start_client<'a>(rx: Receiver<ClientEvent>, client: &mut Client) {
  while let Ok(event) = rx.recv() {
    if let Err(err) = client.handle_client_event(&event).await {
      log::error!("Could not handle event {:?}: {}", event, err);
    }
  }
}