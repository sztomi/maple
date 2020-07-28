use anyhow::Result;
use log::info;

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
        let mut app = self.app.lock().unwrap();
        app.login_state = LoginState::LoggingIn;
        self.plextv.get_auth_token().await
      }
    }
  }
}