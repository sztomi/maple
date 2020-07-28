use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread;

use anyhow::Result;
use log::{info, error};
use tokio::sync::Mutex;

mod network;
mod appstate;
mod ui;
mod logging;

use network::{Network, NetworkEvent};
use appstate::{App, LoginState};
use ui::run_ui;
use logging::set_up_logging;


fn main() -> Result<()> {
  set_up_logging();
  info!("Maple for Plex starting");
  // this architecture is largely based on https://keliris.dev/improving-spotify-tui/ (<3)
  let app = Arc::new(Mutex::new(App::new(LoginState::LoggedOut)));
  let cloned_app = Arc::clone(&app);
  let (tx, rx) = channel::<NetworkEvent>();

  thread::spawn(move || {
    let mut network = Network::new(&app).unwrap();
    start_network(rx, &mut network);
  });

  run_ui(&cloned_app, &tx)
}

#[tokio::main]
async fn start_network<'a>(rx: Receiver<NetworkEvent>, network: &mut Network) {
  while let Ok(event) = rx.recv() {
    if let Err(err) = network.handle_network_event(&event).await {
      error!("Could not handle event {:?}: {}", event, err);
    }
  }
}