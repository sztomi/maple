use std::rc::Rc;

use anyhow::Result;
use slint::{self, Weak};
use tokio::time::Duration;
use webbrowser;

use crate::{app::AppEvent, errors::ClientError, MainWindow, MenuItemData};
use common::config;
use plextvapi::{
  errors::{ApiError::Unauthorized, ApiErrors},
  types::{Resource, Service},
  PlexTvClient, PLEXTV,
};

pub struct Client {
  plextv: PlexTvClient,
  window: Weak<MainWindow>,
  resources: Vec<Resource>,
}

enum Screen {
  LoginScreen = 0,
  MainScreen = 1,
}

impl Client {
  pub fn new(window: Weak<MainWindow>) -> Result<Self> {
    Ok(Client {
      plextv: PlexTvClient::new(&PLEXTV)?,
      window,
      resources: Vec::new(),
    })
  }

  pub async fn handle_app_event(&mut self, event: &AppEvent) -> Result<(), ClientError> {
    match event {
      AppEvent::LoginRequested => self.on_login_requested().await,
      AppEvent::Started => self.on_started().await,
      AppEvent::LogoutRequested => Ok(()),
      AppEvent::MenuItemClicked(index) => Ok(self.on_menu_item_clicked(*index).await),
    }
  }

  pub async fn handle_api_error(&mut self, errors: &ApiErrors) {
    for error in errors.iter() {
      match error {
        Unauthorized => {
          log::info!("Token invalid, retrying login.");
          self.plextv.set_token(None);
          self.set_screen(Screen::LoginScreen)
        }
        _ => (),
      }
    }
  }

  fn set_screen(&mut self, screen: Screen) {
    self.window.upgrade_in_event_loop(|window| {
      window.set_selected_screen(screen as i32);
    });
  }

  async fn on_started(&mut self) -> Result<(), ClientError> {
    let token = config::get("plextv", "token")?;
    if token.is_some() {
      self.plextv.set_token(token);
      self.set_screen(Screen::MainScreen);
      self.fill_sidebar().await?;
    }
    Ok(())
  }

  async fn on_login_requested(&mut self) -> Result<(), ClientError> {
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
          config::set("plextv", "token", &pinf.auth_token.as_deref().unwrap())?;
          self.plextv.set_token(pinf.auth_token);
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

  async fn fill_sidebar(&mut self) -> Result<(), ClientError> {
    log::trace!("Getting resources");
    self.resources = self.plextv.get_resources(true, true, true).await?;
    let resources = self.resources.clone();
    self.window.upgrade_in_event_loop(move |window| {
      let mut items = Vec::new();
      for (idx, res) in resources.iter().enumerate() {
        if !res.provides.contains(&Service::Server) {
          continue;
        }
        items.push(MenuItemData {
          index: idx as i32,
          title: res.name.clone().into(),
          is_sub: false,
        });
      }
      let items_model = Rc::new(slint::VecModel::from(items));
      window.set_menu_items(items_model.clone().into());
    });
    Ok(())
  }

  async fn on_menu_item_clicked(&self, index: i32) {
    log::info!("item was {:?}", self.resources[index as usize]);
    self.window.upgrade_in_event_loop(move |window| {
      window.set_menu_active(index);
    });
  }
}
