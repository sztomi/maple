use plextvapi::{serverclient::ServerClient, types::Resource};

pub(crate) struct ServerItem {
  resource: Resource,
  pub client: ServerClient,
}

pub(crate) struct LibraryItem {
  // tbd
}

pub(crate) enum Item {
  Server(ServerItem),
  Library(LibraryItem)
}

impl ServerItem {
  pub(crate) fn new(resource: Resource, client: ServerClient) -> Self {
    Self { resource, client }
  }
}
