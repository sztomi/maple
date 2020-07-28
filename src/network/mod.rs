mod types;
#[allow(clippy::module_inception)]
mod network;

pub mod plextvclient;

pub use network::{Network, NetworkEvent};
