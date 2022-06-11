
use common::config::ConfigError;
use plextvapi::RequestError;

#[derive(Debug)]
pub enum ClientError {
  RequestError(RequestError),
  ConfigError(ConfigError),
}

impl From<RequestError> for ClientError {
  fn from(err: RequestError) -> Self {
    ClientError::RequestError(err)
  }
}

impl From<ConfigError> for ClientError {
  fn from(err: ConfigError) -> Self {
    ClientError::ConfigError(err)
  }
}
