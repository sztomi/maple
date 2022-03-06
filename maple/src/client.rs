use anyhow::Result;
use webbrowser;
use tokio::time::Duration;

use plextvapi::{PlexTvClient, PLEXTV};
use crate::app::AppEvent;

pub struct Client {
  plextv: PlexTvClient,
}

impl Client {
  pub fn new() -> Result<Self> {
    Ok(Client {
      plextv: PlexTvClient::new(&PLEXTV)?
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
    let mut token: Option<String> = None;

    if webbrowser::open(&auth_url).is_ok() {
      loop {
        tries += 1;
        let pinf = self.plextv.try_pin(&pin).await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;
        if let Some(tk) = pinf.auth_token {
          token = Some(tk);
          break;
        }
        if tries > MAX_TRIES {
          break; // failed to get a token after exhausting tries
        }
      }
    }
    log::trace!("Got token: {}", token.unwrap());
    Ok(())
  }
}