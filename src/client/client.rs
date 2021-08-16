use anyhow::Result;
use log;

use crate::appstate::{SharedApp, AppState};
use crate::client::plextvclient::PlexTvClient;

const PLEX_TV_URL: &str = "https://plex.tv";

pub struct Client<'a> {
  app: &'a SharedApp,
  plextv: PlexTvClient,
}

#[derive(Debug)]
pub enum ClientEvent {
  Startup,
  Login,
}

impl<'a> Client<'a> {
  pub fn new(app: &'a SharedApp) -> Result<Self> {
    Ok(Self {
      app,
      plextv: PlexTvClient::new(PLEX_TV_URL)?
    })
  }

  pub async fn handle_client_event(&mut self, event: &ClientEvent) -> Result<()> {
    match event {
      ClientEvent::Startup => {
        if self.plextv.has_token() {
          log::debug!("plex.tv client using cached token.");
          let mut app = self.app.lock().unwrap();
          app.app_state = AppState::LoggedIn;
          self.plextv.reset_headers();
          let user = self.plextv.get_user().await.unwrap();
        }
        Ok(())
      },
      ClientEvent::Login => {
        log::info!("Login requested.");
        {
          let mut app = self.app.lock().unwrap();
          app.app_state = AppState::LoggingIn;
        }
        match self.plextv.get_auth_token().await {
          Ok(_) => {
            let mut app = self.app.lock().unwrap();
            app.app_state = AppState::LoggedIn;
          },
          Err(err) => {
            log::error!("Could not get plex.tv auth token: {:?}", err);
            let mut app = self.app.lock().unwrap();
            app.app_state = AppState::Error;
          }
        }
        Ok(())
      }
    }
  }
}