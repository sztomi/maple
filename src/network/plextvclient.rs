use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header;
use serde::de::DeserializeOwned;
use anyhow::Result;
use webbrowser;
use tokio::time;


use crate::network::types::*;

const APP_PLEXTV: &str = "https://app.plex.tv";

pub struct PlexTvClient {
  base_url: String,
  token: Option<String>,
  client: reqwest::Client
}

impl PlexTvClient {
  pub fn new(base_url: &'static str) -> Result<Self> {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert("X-Plex-Client-Identifier", "6".parse().unwrap());

    Ok(Self {
      base_url: base_url.to_owned(),
      token: None,
      client: reqwest::Client::builder().default_headers(headers).build()?,
    })
  }

  pub async fn get_auth_token(&self) -> Result<()> {
    let resp = self.post::<CreatePinResponse>("/api/v2/pins?strong=true").await?;
    let auth_url = format!("{}/auth#?clientID=6&code={}", APP_PLEXTV, resp.code);

    if webbrowser::open(&auth_url).is_ok() {
      loop {
          let pin_try_url = format!("/api/v2/pins/{}", resp.id);
          let pinf = self.get::<PinInfo>(&pin_try_url).await?;
          tokio::time::delay_for(Duration::from_millis(1000)).await;
          if let Some(token) = pinf.auth_token {
            println!("GOT TOKEN: {}", token);
            break;
          }
      }
    }

    Ok(())
  }

  // pub fn get_resources(&self) -> Vec<Resource> {
  //   unimplemented!()
  // }

  async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let req_addr = format!("{}/{}", &self.base_url, path);
    let res: T = self.client.get(&req_addr).send().await?.json().await?;
    Ok(res)
  }

  async fn post<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let req_addr = format!("{}/{}", &self.base_url, path);
    let res: T = self.client.post(&req_addr).send().await?.json().await?;
    Ok(res)
  }
}