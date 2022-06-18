use crate::{
  apiclient::ApiClient,
  types::{Connection, Resource, MediaProvider, MediaContainerRoot}, RequestError,
};
use reqwest::header::{self, HeaderMap, HeaderValue};
use thiserror::Error;

pub struct ServerClient {
  token: String,
  client: ApiClient,
  is_relay: bool,
}

#[derive(Debug, Error)]
pub enum ServerClientError {
  #[error("Could not find usable connection to server")]
  NoConnection,
  #[error("No access token provided for server")]
  NoToken,
  #[error("An error occured while performing a request")]
  RequestError(RequestError),
}

fn create_default_headers(token: &str, client_id: &'static str) -> HeaderMap {
  let mut headers = HeaderMap::new();
  headers.insert(
    header::CONTENT_TYPE,
    HeaderValue::from_static("application/x-www-form-urlencoded"),
  );
  headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
  headers.insert(
    "X-Plex-Client-Identifier",
    HeaderValue::from_static(client_id),
  );
  headers.insert("X-Plex-Product", HeaderValue::from_static("Maple for Plex"));
  headers.insert("X-Plex-Token", HeaderValue::from_str(token).ok().unwrap());
  headers
}

impl ServerClient {
  pub fn new(
    server_resource: &Resource,
    client_id: &'static str,
  ) -> Result<Self, ServerClientError> {
    let conn = Self::get_best_connection(&server_resource.connections)?;
    let token = server_resource
      .access_token
      .as_ref()
      .ok_or(ServerClientError::NoToken)?;

    Ok(Self {
      token: token.to_string(),
      client: ApiClient::new(&conn.uri, create_default_headers(token, client_id)),
      is_relay: conn.relay,
    })
  }

  fn get_best_connection(connections: &[Connection]) -> Result<&Connection, ServerClientError> {
    // prefer local connections
    let filtered_conns: Vec<&Connection> = connections.iter().filter(|conn| conn.local).collect();
    if !filtered_conns.is_empty() {
      return Ok(filtered_conns[0]);
    }

    // if no local connection is available, prefer direct
    let filtered_conns: Vec<&Connection> = connections.iter().filter(|conn| !conn.relay).collect();
    if !filtered_conns.is_empty() {
      return Ok(filtered_conns[0]);
    }

    // if all else fails, try to find a relay connection
    let filtered_conns: Vec<&Connection> = connections.iter().filter(|conn| conn.relay).collect();
    if !filtered_conns.is_empty() {
      return Ok(filtered_conns[0]);
    }

    Err(ServerClientError::NoConnection)
  }

  pub async fn get_media_providers(&self) -> Result<Vec<MediaProvider>, ServerClientError> {
    let result = self.client.get::<MediaContainerRoot>("/media/providers", None).await;
    match result {
      Ok(root) => Ok(root.media_container.media_provider),
      Err(err) => Err(ServerClientError::RequestError(err))
    }
  }
}
