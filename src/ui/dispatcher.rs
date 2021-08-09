use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::stringify;

use qmetaobject::prelude::*;
use qmetaobject::qtdeclarative::qml_register_enum;
use cstr::cstr;

use crate::network::NetworkEvent;
use crate::appstate::{SharedApp, LoginState};


#[derive(QObject)]
pub(super) struct Dispatcher {
  tx: Sender<NetworkEvent>,
  app: SharedApp,
  base: qt_base_class!(trait QObject),
  begin_login: qt_method!(fn(&self)),
}

macro_rules! event_sender {
  ($func:ident -> $event:ident) => {
    fn $func(&self) {
      log::trace!("Sending NetworkEvent::{}", stringify!($event));
      if let Err(err) = self.tx.send(NetworkEvent::$event) {
        log::error!("Could not send internal event {}: {}", stringify!($event), err);
      }
    }
  };
}

impl Dispatcher {
  pub(super) fn new(tx: Sender<NetworkEvent>, app: &SharedApp) -> Self {
    Self {
      tx,
      app: Arc::clone(&app),
      base: Default::default(),
      begin_login: Default::default(),
    }
  }

  pub(super) fn register_qt_types() {
    qml_register_enum::<LoginState>(cstr!("MapleNative"), 1, 0, cstr!("LoginState"));
  }

  event_sender! { begin_login -> Login }
}