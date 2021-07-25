use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread;

use anyhow::Result;
use log;
use tokio::sync::Mutex;

mod network;
mod appstate;
mod ui;
mod logging;
mod config;

use network::{Network, NetworkEvent};
use appstate::{App, LoginState};
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
  let app = Arc::new(Mutex::new(App::new(LoginState::LoggedOut)));
  let cloned_app = Arc::clone(&app);
  let (tx, rx) = channel::<NetworkEvent>();

  thread::spawn(move || {
    let mut network = Network::new(&app).unwrap();
    start_network(rx, &mut network);
  });

  run_ui(&cloned_app, tx)
}

#[tokio::main]
async fn start_network<'a>(rx: Receiver<NetworkEvent>, network: &mut Network) {
  while let Ok(event) = rx.recv() {
    if let Err(err) = network.handle_network_event(&event).await {
      log::error!("Could not handle event {:?}: {}", event, err);
    }
  }
}