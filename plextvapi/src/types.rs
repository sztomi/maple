use std::str::FromStr;

use serde::Deserialize;
use serde_aux::prelude::*;

use crate::errors::InternalClientError;

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

// The `provides` field items for `Resource`
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Service {
  Server,
  Client,
  Player,
  PubSubPlayer,
  Controller,
  SyncTarget,
  ProviderPlayback,
}

impl FromStr for Service {
  type Err = InternalClientError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "server" => Ok(Service::Server),
      "client" => Ok(Service::Client),
      "player" => Ok(Service::Player),
      "pubsub-player" => Ok(Service::PubSubPlayer),
      "controller" => Ok(Service::Controller),
      "sync-target" => Ok(Service::SyncTarget),
      "provider-playback" => Ok(Service::ProviderPlayback),
      _ => Err(InternalClientError::UnparseableService((&s).to_string())),
    }
  }
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
  #[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
  pub provides: Vec<Service>,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaContainerRoot {
  #[serde(rename = "MediaContainer")]
  pub media_container: MediaContainer,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaContainer {
  pub size: i32,
  pub allow_camera_upload: bool,
  pub allow_channel_access: bool,
  pub allow_media_deletion: bool,
  pub allow_sharing: bool,
  pub allow_sync: bool,
  pub allow_tuners: bool,
  pub audiobook: i32,
  pub background_processing: bool,
  pub certificate: bool,
  pub companion_proxy: bool,
  pub country_code: String,
  pub diagnostics: String,
  pub event_stream: bool,
  pub friendly_name: String,
  pub livetv: i32,
  pub machine_identifier: String,
  pub music_analysis: i32,
  pub my_plex: bool,
  pub my_plex_mapping_state: String,
  pub my_plex_signin_state: String,
  pub my_plex_subscription: bool,
  pub my_plex_username: String,
  pub offline_transcode: i32,
  pub owner_features: String,
  pub photo_auto_tag: bool,
  pub platform: String,
  pub platform_version: String,
  pub plugin_host: bool,
  pub push_notifications: bool,
  pub read_only_libraries: bool,
  #[serde(rename = "streamingBrainABRVersion")]
  pub streaming_brain_abrversion: i32,
  pub streaming_brain_version: i32,
  pub sync: bool,
  pub transcoder_active_video_sessions: i32,
  pub transcoder_audio: bool,
  pub transcoder_lyrics: bool,
  pub transcoder_subtitles: bool,
  pub transcoder_video: bool,
  pub transcoder_video_bitrates: String,
  pub transcoder_video_qualities: String,
  pub transcoder_video_resolutions: String,
  pub updated_at: i32,
  pub updater: bool,
  pub version: String,
  pub voice_search: bool,
  #[serde(rename = "MediaProvider")]
  pub media_provider: Vec<MediaProvider>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaProvider {
  pub identifier: String,
  pub title: String,
  pub types: String,
  pub protocols: String,
  #[serde(rename = "Feature")]
  pub features: Vec<Feature>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
  pub key: Option<String>,
  #[serde(rename = "type")]
  pub type_field: String,
  #[serde(rename = "Directory")]
  pub directory: Option<Vec<Directory>>,
  #[serde(rename = "Action")]
  #[serde(default)]
  pub action: Vec<Action>,
  pub flavor: Option<String>,
  pub scrobble_key: Option<String>,
  pub unscrobble_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
  pub hub_key: Option<String>,
  pub title: String,
  pub agent: Option<String>,
  pub language: Option<String>,
  pub refreshing: Option<bool>,
  pub scanner: Option<String>,
  pub uuid: Option<String>,
  pub id: Option<String>,
  pub key: Option<String>,
  #[serde(rename = "type")]
  pub type_field: Option<String>,
  pub updated_at: Option<i32>,
  pub scanned_at: Option<i32>,
  #[serde(rename = "Pivot")]
  #[serde(default)]
  pub pivot: Vec<Pivot>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pivot {
  pub id: String,
  pub key: String,
  #[serde(rename = "type")]
  pub type_field: String,
  pub title: String,
  pub context: String,
  pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
  pub id: String,
  pub key: String,
}
