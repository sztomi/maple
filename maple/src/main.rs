slint::include_modules!();

use std::thread;
use std::sync::Arc;

use log;
use flume::{self, Receiver};
use tokio;
use anyhow::Result;

use common::{logging::setup_logging, config::get_config_file};


mod app;
mod client;

use app::AppEvent;
use client::Client;


fn main() -> Result<()> {
  setup_logging();
  log::info!("Maple for Plex starting");
  log::info!("Using config file: {}", get_config_file()?.display());

  let (tx, rx) = flume::unbounded();
  let tx = Arc::new(tx);

  let mainwindow = MainWindow::new();
  let mainwindow_weak = mainwindow.as_weak();

  let tx_1 = tx.clone();
  mainwindow.on_login_clicked(move || {
    log::info!("Login clicked!");
    tx_1.clone().send(AppEvent::LoginRequested);
  });

  thread::spawn(move || {
    let mut client = Client::new(mainwindow_weak).unwrap();
    start_client(rx, &mut client);
  });

  tx.send(AppEvent::Started)?;

  mainwindow.run();
  Ok(())
}


#[tokio::main]
async fn start_client(rx: Receiver<AppEvent>, client: &mut Client) {
  while let Ok(event) = rx.recv() {
    if let Err(err) = client.handle_app_event(&event).await {
      log::error!("Could not handle event {:?}: {}", event, err);
    }
  }
}