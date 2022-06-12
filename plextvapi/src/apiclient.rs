use reqwest::{self, header::HeaderMap};
use serde::de::DeserializeOwned;

use crate::{errors::PlexTvErrors, RequestError};

pub(crate) type QueryParams<'a> = Vec<(&'a str, &'a str)>;

/// Generic API client
pub(crate) struct ApiClient {
  base_url: String,
  client: reqwest::Client,
}

impl ApiClient {
  pub(crate) fn new(base_url: &str, default_headers: HeaderMap) -> Self {
    let client = reqwest::Client::builder().default_headers(default_headers).build();
    match client {
      Ok(client) => Self { base_url: base_url.to_string(), client },
      Err(err) => {
        log::error!("fatal error: Could not create client: {:?}", err);
        panic!("Cannot continue without a client");
      }
    }

  }

  pub(crate) fn reset_headers(&mut self, headers: HeaderMap) {
    let client = reqwest::Client::builder().default_headers(headers).build();
    match client {
      Ok(client) => self.client = client,
      Err(err) => {
        log::error!("Could not create reqwest client: {}", err);
        panic!("Fatal error: cannot continue without a client!");
      }
    }
  }

  pub(crate) async fn get<T: DeserializeOwned>(
    &self,
    path: &str,
    params: Option<QueryParams<'_>>,
  ) -> Result<T, RequestError> {
    let req_addr = format!("{}/{}", &self.base_url, path);
    log::trace!("GET {}", req_addr);
    let mut builder = self.client.get(&req_addr);
    if let Some(params) = params {
      builder = builder.query(&params);
    }
    let resp = builder.send().await?;

    let status = resp.status();
    let resp_text = resp.text().await.map_err(|e| RequestError::SendError(e))?;

    if !status.is_success() {
      let deser_errs = serde_json::from_str::<PlexTvErrors>(&resp_text);
      match deser_errs {
        Ok(errors) => {
          for err in errors.errors.iter() {
            log::error!("Error from {}: {:?}", path, err);
          }
          return Err(RequestError::Error(
            errors
              .iter()
              .map(|err| err.try_into())
              .filter_map(|item| {
                if !item.is_ok() {
                  log::warn!("Could not convert PlexTvError error to ApiError!");
                  return None;
                }
                Some(item.unwrap())
              })
              .collect(),
          ));
        }
        Err(err) => {
          log::error!(
            "Could not deserialize error response. Text was: {}",
            resp_text
          );
          return Err(RequestError::DeserError(err));
        }
      }
    }

    let deser_result = serde_json::from_str::<T>(&resp_text);
    match deser_result {
      Ok(val) => Ok(val),
      Err(err) => {
        log::trace!("Could not decode json: {}", resp_text);
        Err(RequestError::DeserError(err))
      }
    }
  }

  pub(crate) async fn post<T: DeserializeOwned>(&self, path: &str) -> Result<T, RequestError> {
    let req_addr = format!("{}/{}", &self.base_url, path);
    log::trace!("POST {}", req_addr);
    let builder = self.client.post(&req_addr);
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
          return Err(RequestError::Error(
            errors
              .iter()
              .map(|err| err.try_into())
              .filter_map(|item| {
                if !item.is_ok() {
                  log::warn!("Could not convert PlexTvError error to ApiError!");
                  return None;
                }
                Some(item.unwrap())
              })
              .collect(),
          ));
        }
        Err(err) => {
          log::error!(
            "Could not deserialize error response. Text was: {}",
            resp_text
          );
          return Err(RequestError::DeserError(err));
        }
      }
    }

    let deser_result = serde_json::from_str::<T>(&resp_text);
    match deser_result {
      Ok(val) => Ok(val),
      Err(err) => {
        log::trace!("Could not decode json: {}", resp_text);
        Err(RequestError::DeserError(err))
      }
    }
  }
}
