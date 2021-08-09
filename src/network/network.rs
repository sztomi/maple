use anyhow::Result;
use log;

use crate::appstate::{SharedApp, AppState};
use crate::network::plextvclient::PlexTvClient;

const PLEX_TV_URL: &str = "https://plex.tv";

pub struct Network<'a> {
  app: &'a SharedApp,
  plextv: PlexTvClient,
}

#[derive(Debug)]
pub enum NetworkEvent {
  Startup,
  Login,
}

impl<'a> Network<'a> {
  pub fn new(app: &'a SharedApp) -> Result<Self> {
    Ok(Self {
      app,
      plextv: PlexTvClient::new(PLEX_TV_URL)?
    })
  }

  pub async fn handle_network_event(&mut self, event: &NetworkEvent) -> Result<()> {
    match event {
      NetworkEvent::Startup => {
        if self.plextv.has_token() {
          log::debug!("plex.tv client using cached token.");
          let mut app = self.app.lock().unwrap();
          app.app_state = AppState::LoggedIn;
          self.plextv.reset_headers();
          let user = self.plextv.get_user().await.unwrap();
        }
        Ok(())
      },
      NetworkEvent::Login => {
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