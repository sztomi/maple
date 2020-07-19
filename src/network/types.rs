#[macro_use]
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Connection {
  pub protocol: String,
  pub address: String,
  pub port: String,
  pub uri: String,
  pub local: bool,
  pub relay: bool,
  #[serde(rename = "IPv6")]
  pub ipv6: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
  pub name: String,
  pub product: String,
  pub product_version: String,
  pub platform: String,
  pub platform_version: String,
  pub device: String,
  pub client_identifier: String,
  pub created_at: String,
  pub last_seen_at: String,
  pub provides: String,
  pub owner_id: String,
  pub source_title: String,
  pub public_address: String,
  pub access_token: String,
  pub owned: bool,
  pub home: bool,
  pub presence: bool,
  pub synced: bool,
  pub relay: bool,
  pub dns_rebinding_protection: bool,
  pub https_required: bool,
  pub public_address_matches: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PinLocation {
  pub code: String,
  pub country: String,
  pub city: String,
  pub subdivisions: String,
  pub coordinates: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatePinResponse {
  pub id: u32,
  pub code: String,
  pub product: String,
  pub trusted: bool,
  pub origin: Option<String>,
  pub client_identifier: String,
  pub location: PinLocation,
  pub created_at: String,
  pub expires_at: String,
  pub expires_in: u32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PinInfo {
  pub id: u32,
  pub code: String,
  pub product: String,
  pub trusted: bool,
  pub origin: Option<String>,
  pub client_identifier: String,
  pub location: PinLocation,
  pub created_at: String,
  pub expires_at: String,
  pub expires_in: u32,
  pub auth_token: Option<String>,
  pub new_registration: Option<bool>,
}