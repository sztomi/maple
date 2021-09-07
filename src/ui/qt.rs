use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::stringify;

use anyhow::Result;
use cpp::cpp;
use qmetaobject::prelude::*;
use qmetaobject::qml_register_enum;
use log;
use cstr::cstr;

use crate::appstate::{SharedApp, AppState};
use crate::client::ClientEvent;

qrc!(qml_resources_init,
  "" {
      "src/qml/main.qml",
      "src/qml/mpv.qml",
      "src/qml/elements/Button.qml",
      "src/qml/elements/Style.qml",
      "src/qml/elements/qmldir",
      "src/qml/views/LoginView.qml",
      "src/qml/views/MainView.qml",
      "src/qml/views/qmldir",
  },
);

cpp! {{
  #include <QtQml/qqml.h>
  #include "src/cpp/mpvobject.h"
  #include "src/cpp/mpvobject.cpp"
  #include "src/cpp/mpvobject.moc.cpp"
}}



#[derive(QObject)]
pub(super) struct Dispatcher {
  tx: Sender<ClientEvent>,
  app: SharedApp,
  base: qt_base_class!(trait QObject),
  begin_login: qt_method!(fn(&self)),
  app_state: qt_property!(i32; READ get_app_state NOTIFY app_state_changed),
  app_state_changed: qt_signal!(),
  get_app_state: qt_method!(fn(&self) -> i32),
}

macro_rules! event_sender {
  ($func:ident -> $event:ident) => {
    fn $func(&self) {
      log::trace!("Sending NetworkEvent::{}", stringify!($event));
      if let Err(err) = self.tx.send(ClientEvent::$event) {
        log::error!("Could not send internal event {}: {}", stringify!($event), err);
      }
    }
  };
}

impl Dispatcher {
  pub(super) fn new(tx: Sender<ClientEvent>, app: &SharedApp) -> Self {
    Self {
      tx,
      app: Arc::clone(&app),
      base: Default::default(),
      begin_login: Default::default(),
      app_state: Default::default(),
      app_state_changed: Default::default(),
      get_app_state: Default::default(),
    }
  }

  pub(super) fn register_qt_types() {
    qml_register_enum::<AppState>(cstr!("MapleNative"), 1, 0, cstr!("AppState"));
  }

  pub(super) fn get_app_state(&self) -> i32 {
    let app = self.app.lock().unwrap();
    app.app_state as i32
  }

  event_sender! { begin_login -> Login }
}


pub fn run_ui<'a>(app: &'a SharedApp, tx: Sender<ClientEvent>) -> Result<()> {
  tx.send(ClientEvent::Startup)?;
  log::info!("Starting GUI");
  qml_resources_init();
  unsafe {
    cpp!([] -> () as "void" {
      qmlRegisterType<MpvObject>("mpvtest", 1, 0, "MpvObject");
    });
  }
  let mut engine = QmlEngine::new();
  Dispatcher::register_qt_types();
  let dispatcher = QObjectBox::new(Dispatcher::new(tx, app));
  engine.set_object_property("dispatcher".into(), dispatcher.pinned());
  engine.load_file("qrc:/src/qml/main.qml".into());
  engine.exec();
  Ok(())
}
