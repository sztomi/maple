use anyhow::{bail, Result};
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::types::*;
use common::config;

pub const PLEXTV: &str = "https://plex.tv";
pub const APP_PLEXTV: &str = "https://app.plex.tv";
const CLIENT_ID: &str = "Maple_1_0";

type QueryParams<'a> = Vec<(&'a str, &'a str)>;

pub struct PlexTvClient {
  base_url: String,
  token: Option<String>,
  client: reqwest::Client,
}

#[derive(Error, Debug)]
enum RequestError {
  #[error("plex.tv returned one or more errors")]
  Error(PlexTvErrors),
  #[error("Error while sending request")]
  SendError(reqwest::Error),
  #[error("Deserialization failed.")]
  DeserError(serde_json::Error),
}

fn create_default_headers(token: Option<String>) -> Result<HeaderMap> {
  let mut headers = HeaderMap::new();
  headers.insert(
    header::CONTENT_TYPE,
    HeaderValue::from_static("application/x-www-form-urlencoded"),
  );
  headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
  headers.insert(
    "X-Plex-Client-Identifier",
    HeaderValue::from_static(CLIENT_ID),
  );
  headers.insert("X-Plex-Product", HeaderValue::from_static("Maple for Plex"));
  if let Some(tk) = token {
    headers.insert("X-Plex-Token", HeaderValue::from_str(&tk)?);
  }
  Ok(headers)
}

impl PlexTvClient {
  pub fn new(base_url: &'static str) -> Result<Self> {
    let headers = create_default_headers(None)?;

    Ok(Self {
      base_url: base_url.to_owned(),
      token: config::get("plextv", "token")?,
      client: reqwest::Client::builder()
        .default_headers(headers)
        .build()?,
    })
  }

  pub fn has_token(&self) -> bool {
    self.token.is_some()
  }

  pub fn reset_headers(&mut self) -> Result<()> {
    let mut token: Option<String> = None;
    if self.token.is_some() {
      token = Some(self.token.as_deref().unwrap().to_string());
    }
    let headers = create_default_headers(token)?;
    self.client = reqwest::Client::builder()
      .default_headers(headers)
      .build()?;
    Ok(())
  }

  /// Creates a pin that can be used for authentication.
  ///
  /// # Arguments
  ///
  /// * `strong`: when false, it will return a 4-letter code to be entered by humans
  ///
  /// # How to obtain a plex.tv token
  ///
  /// After receiving a PIN, the user has to be presented with a plex.tv authentication challenge.
  /// On devices that support it, this should be a browser window that navigates to plex.tv
  /// for authenticating. On other devices, a link code should be displayed and the user
  /// asked to visit plex.tv/link
  ///
  /// After that, the client should periodically (e.g. every 1-2 seconds) call try_pin until it
  /// succeeds or a certain number of tries are exhausted.
  ///
  /// One way to implement that with a browser is the following snippet:
  ///
  /// ```
  /// let pin = self.plextv.create_pin(true).await?;
  /// let auth_url = self.plextv.get_auth_url(&pin);
  /// let mut tries = 0;
  /// const MAX_TRIES: u16 = 128;
  /// let mut token: Option<String> = None;
  /// if webbrowser::open(&auth_url).is_ok() {
  ///   loop {
  ///     tries += 1;
  ///     let pinf = self.plextv.try_pin(&pin).await?;
  ///     tokio::time::sleep(Duration::from_millis(1000)).await;
  ///     if let Some(tk) = pinf.auth_token {
  ///       token = Some(tk);
  ///       self.window.upgrade_in_event_loop(|window| {
  ///         window.set_selected_screen(1);
  ///       });
  ///       break;
  ///     }
  ///     if tries > MAX_TRIES {
  ///       break; // failed to get a token after exhausting tries
  ///     }
  ///   }
  /// }
  /// ```
  pub async fn create_pin(&mut self, strong: bool) -> Result<CreatePinResponse> {
    if self.token.is_some() {
      log::debug!("Getting new token despite a cached one existing.");
    }
    let resp = self
      .post::<CreatePinResponse>(&format!("/api/v2/pins?strong={}", strong))
      .await?;
    Ok(resp)
  }

  pub fn get_auth_url(&self, pin: &CreatePinResponse) -> String {
    format!(
      "{}/auth#?clientID={}&code={}",
      APP_PLEXTV, CLIENT_ID, pin.code
    )
    .to_string()
  }

  pub async fn try_pin(&self, pin: &CreatePinResponse) -> Result<PinInfo> {
    self
      .get::<PinInfo>(&format!("/api/v2/pins/{}", pin.id), None)
      .await
  }

  pub async fn get_resources(
    &self,
    include_https: bool,
    include_relay: bool,
    include_ipv6: bool,
  ) -> Result<Vec<Resource>> {
    use common::conversion::bool_to_num;
    let resources = self
      .get::<Vec<Resource>>(
        "/api/v2/resources",
        Some(vec![
          ("includeHttps", &bool_to_num(include_https).to_string()),
          ("includeRelay", &bool_to_num(include_relay).to_string()),
          ("includeIPv6", &bool_to_num(include_ipv6).to_string()),
        ]),
      )
      .await?;
    Ok(resources)
  }

  pub async fn get_user(&self) -> Result<User> {
    let user = match self.get::<User>("/api/v2/user", None).await {
      Ok(val) => val,
      Err(err) => {
        log::error!("{:?}", err);
        bail!(err);
      }
    };
    Ok(user)
  }

  async fn get<T: DeserializeOwned>(
    &self,
    path: &str,
    params: Option<QueryParams<'_>>,
  ) -> Result<T> {
    let req_addr = format!("{}/{}", &self.base_url, path);
    log::trace!("GET {}", req_addr);
    let mut builder = self.client.get(&req_addr);
    if let Some(params) = params {
      builder = builder.query(&params);
    }
    let resp = builder
      .send()
      .await
      .map_err(|e| RequestError::SendError(e))?;

    let status = resp.status();
    let resp_text = resp.text().await?;

    if !status.is_success() {
      let deser_errs = serde_json::from_str::<PlexTvErrors>(&resp_text);
      match deser_errs {
        Ok(errors) => {
          for err in errors.errors.iter() {
            log::error!("Error from {}: {:?}", path, err);
          }
          bail!(RequestError::Error(errors));
        }
        Err(_) => {
          log::error!(
            "Could not deserialize error response. Text was: {}",
            resp_text
          );
          bail!("ugh");
        }
      }
    }

    let deser_result = serde_json::from_str::<T>(&resp_text);
    match deser_result {
      Ok(val) => Ok(val),
      Err(err) => {
        log::trace!("Could not decode json: {}", resp_text);
        bail!(err);
      }
    }
  }

  async fn post<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let req_addr = format!("{}/{}", &self.base_url, path);
    log::trace!("POST {}", req_addr);
    let mut builder = self.client.post(&req_addr);
    let resp = builder
      .send()
      .await
      .map_err(|e| RequestError::SendError(e))?;

    let status = resp.status();
    let resp_text = resp.text().await?;

    if !status.is_success() {
      let deser_errs = serde_json::from_str::<PlexTvErrors>(&resp_text);
      match deser_errs {
        Ok(errors) => {
          for err in errors.errors.iter() {
            log::error!("Error from {}: {:?}", path, err);
          }
          bail!(RequestError::Error(errors));
        }
        Err(_) => {
          log::error!(
            "Could not deserialize error response. Text was: {}",
            resp_text
          );
          bail!("ugh");
        }
      }
    }

    let deser_result = serde_json::from_str::<T>(&resp_text);
    match deser_result {
      Ok(val) => Ok(val),
      Err(err) => {
        log::trace!("Could not decode json: {}", resp_text);
        bail!(err);
      }
    }
  }
}
