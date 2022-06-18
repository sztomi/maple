slint::include_modules!();

use std::sync::Arc;
use std::thread;

use anyhow::Result;
use flume::{self, Receiver};

use common::{config::get_config_file, logging::setup_logging};

mod app;
mod client;
mod errors;
mod constants;
mod viewmodel;

use app::AppEvent;
use client::Client;

use crate::errors::ClientError;

fn main() -> Result<()> {
  setup_logging();
  log::info!("Maple for Plex starting");
  log::info!("Using config file: {}", get_config_file()?.display());

  let (tx, rx) = flume::unbounded();
  let tx = Arc::new(tx);

  let mainwindow = MainWindow::new();

  // TODO(sztomi): probably macro this pattern
  let tx_1 = tx.clone();
  mainwindow.on_login_clicked(move || {
    log::info!("Login clicked!");
    tx_1.clone().send(AppEvent::LoginRequested).ok();
  });

  let tx_2 = tx.clone();
  mainwindow.on_logout_clicked(move || {
    log::info!("Logout clicked!");
    tx_2.clone().send(AppEvent::LogoutRequested).ok();
  });

  let tx_3 = tx.clone();
  mainwindow.on_menu_item_clicked(move |index| {
    log::info!("menu item {} clicked!", index);
    tx_3.clone().send(AppEvent::MenuItemClicked(index)).ok();
  });

  let mainwindow_weak = mainwindow.as_weak();
  thread::spawn(move || {
    let mut client = Client::new(mainwindow_weak).unwrap();
    start_client(rx, &mut client);
  });

  tx.send(AppEvent::Started)?;

  mainwindow.run();
  Ok(())
}

#[tokio::main]
#[allow(clippy::collapsible_match)]
async fn start_client(rx: Receiver<AppEvent>, client: &mut Client) {
  while let Ok(event) = rx.recv() {
    if let Err(err) = client.handle_app_event(&event).await {
      log::error!("Could not handle event {:?}: {:?}", event, err);
      if let ClientError::RequestError(err) = err {
        if let plextvapi::RequestError::Error(apierrs) = err {
          client.handle_api_error(&apierrs).await
        }
      }
    }
  }
}
