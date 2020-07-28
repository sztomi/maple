use anyhow::Result;
use log::{info, error};

use crate::appstate::{SharedApp, LoginState};
use crate::network::plextvclient::PlexTvClient;

const PLEX_TV_URL: &str = "https://plex.tv";

pub struct Network<'a> {
  app: &'a SharedApp,
  plextv: PlexTvClient,
}

#[derive(Debug)]
pub enum NetworkEvent {
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
      NetworkEvent::Login => {
        info!("Login requested.");
        {
          let mut app = self.app.lock().await;
          app.login_state = LoginState::LoggingIn;
        }
        match self.plextv.get_auth_token().await {
          Ok(_) => {
            let mut app = self.app.lock().await;
            app.login_state = LoginState::LoggedIn;
          },
          Err(err) => {
            error!("Could not get plex.tv auth token: {:?}", err);
            let mut app = self.app.lock().await;
            app.login_state = LoginState::Error;
          }
        }
        Ok(())
      }
    }
  }
}