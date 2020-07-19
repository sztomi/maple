use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;

mod network;
mod appstate;
mod ui;

use network::{Network, NetworkEvent};
use appstate::{App, LoginState};
use ui::run_ui;


fn main() -> Result<()> {
  let app = Arc::new(Mutex::new(App::new(LoginState::LoggedOut)));
  let cloned_app = Arc::clone(&app);
  let (tx, rx) = channel::<NetworkEvent>();

  thread::spawn(move || {
    let mut network = Network::new(&app);
    start_network(rx, &mut network);
  });

  run_ui(&cloned_app, &tx)
}

#[tokio::main]
async fn start_network<'a>(rx: Receiver<NetworkEvent>, network: &mut Network) {
  while let Ok(event) = rx.recv() {
    network.handle_network_event(event).await
  }
}