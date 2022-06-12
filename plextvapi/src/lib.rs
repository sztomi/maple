pub mod errors;
pub mod plextvclient;
pub mod types;
mod apiclient;

pub use plextvclient::{PlexTvClient, RequestError};
pub use plextvclient::{APP_PLEXTV, PLEXTV};
