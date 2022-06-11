use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Connection {
  pub protocol: String,
  pub address: String,
  pub port: u32,
  pub uri: String,
  pub local: bool,
  pub relay: bool,
  #[serde(rename = "IPv6")]
  pub ipv6: bool,
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

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
  pub name: String,
  pub product: String,
  pub product_version: String,
  pub platform: String,
  pub platform_version: String,
  pub device: Option<String>,
  pub client_identifier: String,
  pub created_at: String,
  pub last_seen_at: String,
  pub provides: String,
  pub source_title: Option<String>,
  pub public_address: String,
  pub access_token: Option<String>,
  pub connections: Vec<Connection>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
  pub active: bool,
  pub subscribed_at: Option<String>,
  pub status: String, // TODO(sztomi): strong type?
  pub plan: String,
  pub payment_service: Option<String>,
  pub features: Vec<String>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Subscription2 {
  #[serde(rename = "type")]
  pub type_: String,
  pub state: String,
  pub mode: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
  pub auto_select_audio: bool,
  pub default_audio_language: Option<String>,
  pub default_subtitle_language: Option<String>,
  pub auto_select_subtitle: u32,
  pub default_subtitle_accessibility: u32,
  pub default_subtitle_forced: u32,
}

// TODO(sztomi): some of these fields  could use strong types maybe?
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub ads_consent: bool,
  pub ads_consent_reminder_at: u64,
  pub ads_consent_set_at: u64,
  pub anonymous: Option<bool>,
  pub auth_token: String,
  pub backup_codes_created: bool,
  pub certificate_version: u32,
  pub confirmed: bool,
  pub country: String,
  pub email: Option<String>,
  pub email_only_auth: bool,
  pub entitlements: Vec<String>,
  pub experimental_features: bool,
  pub guest: bool,
  pub has_password: bool,
  pub home: bool,
  pub home_admin: bool,
  pub home_size: u32,
  pub id: u64,
  pub locale: Option<String>,
  pub mailing_list_active: bool,
  pub mailing_list_status: String,
  pub max_home_size: u32,
  pub profile: Profile,
  pub protected: bool,
  pub remember_expires_at: u64,
  pub restricted: bool,
  pub roles: Vec<String>,
  pub scrobble_types: String,
  pub subscription: Subscription,
  pub subscription_description: String,
  pub thumb: String,
  pub title: String,
  pub two_factor_enabled: bool,
  pub username: String,
  pub uuid: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pivot {
  pub id: String,
  pub key: String,
  #[serde(rename = "type")]
  pub type_desc: String,
  pub title: String,
  pub context: String,
  pub symbol: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
  pub hub_key: String,
  pub title: String,

}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
  #[serde(rename = "type")]
  pub type_desc: String,
  pub key: Option<String>,
}
