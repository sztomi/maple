mod types;
#[allow(clippy::module_inception)]
mod client;

pub mod plextvclient;

pub use client::{Client, ClientEvent};
