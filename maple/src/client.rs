use std::rc::Rc;

use anyhow::Result;
use slint::{self, Weak};
use webbrowser;
use tokio::time::Duration;

use plextvapi::{PlexTvClient, PLEXTV};
use crate::{app::AppEvent, MainWindow, MenuItemData};

pub struct Client {
  plextv: PlexTvClient,
  window: Weak<MainWindow>,
}

impl Client {
  pub fn new(window: Weak<MainWindow>) -> Result<Self> {
    Ok(Client {
      plextv: PlexTvClient::new(&PLEXTV)?,
      window
    })
  }

  pub async fn handle_app_event(&mut self, event: &AppEvent) -> Result<()> {
    match event {
      AppEvent::LoginRequested => self.do_login().await,
    }
  }

  async fn do_login(&mut self) -> Result<()> {
    log::trace!("Calling create_pin");
    let pin = self.plextv.create_pin(true).await?;
    log::trace!("Got pin: {}", pin.code);
    let auth_url = self.plextv.get_auth_url(&pin);
    let mut tries = 0;
    const MAX_TRIES: u16 = 128;

    if webbrowser::open(&auth_url).is_ok() {
      loop {
        tries += 1;
        let pinf = self.plextv.try_pin(&pin).await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;
        if pinf.auth_token.is_some() {
          self.plextv.set_token(pinf.auth_token)?;
          self.window.upgrade_in_event_loop(|window| {
            window.set_selected_screen(1);
          });
          self.fill_sidebar().await?;
          break;
        }
        if tries > MAX_TRIES {
          break; // failed to get a token after exhausting tries
        }
      }
    }
    Ok(())
  }

  async fn fill_sidebar(&self) -> Result<()> {
    log::trace!("Getting resources");
    let resources = self.plextv.get_resources(true, true, true).await?;
    self.window.upgrade_in_event_loop(move |window| {
      let mut items = Vec::new();
      for res in resources {
        items.push(MenuItemData{ title: res.name.into() });
      }
      let items_model = Rc::new(slint::VecModel::from(items));
      window.set_menu_items(items_model.clone().into());
    });
    Ok(())
  }
}