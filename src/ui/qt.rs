use std::sync::mpsc::Sender;
use std::stringify;

use anyhow::Result;
use cpp::cpp;
use qmetaobject::prelude::*;
use log;

use crate::appstate::SharedApp;
use crate::network::NetworkEvent;

qrc!(qml_resources_init,
  "" {
      "src/qml/main.qml",
      "src/qml/mpv.qml",
      "src/qml/elements/Button.qml",
      "src/qml/elements/Style.qml",
      "src/qml/elements/qmldir",
  },
);

cpp! {{
  #include <QtQml/qqml.h>
  #include "src/cpp/mpvobject.h"
  #include "src/cpp/mpvobject.cpp"
  #include "src/cpp/mpvobject.moc.cpp"
}}

#[derive(QObject)]
struct Dispatcher {
  tx: Sender<NetworkEvent>,
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
  fn new(tx: Sender<NetworkEvent>) -> Self {
    Self {
      tx,
      base: Default::default(),
      begin_login: Default::default()
    }
  }

  event_sender! { begin_login -> Login }
}

pub fn run_ui<'a>(_app: &'a SharedApp, tx: Sender<NetworkEvent>) -> Result<()> {
  tx.send(NetworkEvent::Startup)?;
  qml_resources_init();
  unsafe {
    cpp!([] -> () as "void" {
      qmlRegisterType<MpvObject>("mpvtest", 1, 0, "MpvObject");
    });
  }
  let mut engine = QmlEngine::new();
  let dispatcher = QObjectBox::new(Dispatcher::new(tx));
  engine.set_object_property("dispatcher".into(), dispatcher.pinned());
  engine.load_file("qrc:/src/qml/main.qml".into());
  engine.exec();
  Ok(())
}
