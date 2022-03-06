slint::include_modules!();

use std::thread;

use log;
use flume::{self, Receiver};
use tokio;

use common::logging::setup_logging;

mod app;
mod client;

use app::AppEvent;
use client::Client;


fn main() {
  setup_logging();
  let (tx, rx) = flume::unbounded();

  log::info!("Maple for Plex starting");
  let mainwindow = MainWindow::new();

  mainwindow.on_login_clicked(move || {
    log::info!("Login clicked!");
    tx.send(AppEvent::LoginRequested);
  });

  thread::spawn(move || {
    let mut client = Client::new().unwrap();
    start_client(rx, &mut client);
  });

  mainwindow.run();
}


#[tokio::main]
async fn start_client<'a>(rx: Receiver<AppEvent>, client: &mut Client) {
  while let Ok(event) = rx.recv() {
    if let Err(err) = client.handle_app_event(&event).await {
      log::error!("Could not handle event {:?}: {}", event, err);
    }
  }
}