use std::sync::{Arc, Mutex};

use crate::appstate::SharedApp;


pub struct Network<'a> {
  app: &'a SharedApp,
}

pub enum NetworkEvent {
  Login,
}

impl<'a> Network<'a> {
  pub fn new(app: &'a SharedApp) -> Self {
    Self {
      app: app,
    }
  }

  pub async fn handle_network_event(&mut self, event: NetworkEvent) {
    match event {
      NetworkEvent::Login => {
        println!("Login requested.");
      }
    }
  }
}