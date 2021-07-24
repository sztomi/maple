use std::sync::mpsc::Sender;

use anyhow::{Error, Result};
use cpp::cpp;
use qmetaobject::prelude::*;
use log;

use crate::appstate::SharedApp;
use crate::network::NetworkEvent;

qrc!(qml_resources_init,
  "" {
      "qml/main.qml",
      "qml/mpv.qml",
      "qml/elements/Button.qml",
      "qml/elements/Style.qml",
      "qml/elements/qmldir",
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

impl Dispatcher {
  fn new(tx: Sender<NetworkEvent>) -> Self {
    Self {
      tx,
      base: Default::default(),
      begin_login: Default::default()
    }
  }

  fn begin_login(&self) {
    log::trace!("beginLogin called");
    self.tx.send(NetworkEvent::Login);
  }
}

pub fn run_ui<'a>(_app: &'a SharedApp, tx: Sender<NetworkEvent>) -> Result<()> {
  qml_resources_init();
  unsafe {
    cpp!([] -> () as "void" {
      qmlRegisterType<MpvObject>("mpvtest", 1, 0, "MpvObject");
    });
  }
  let mut engine = QmlEngine::new();
  let dispatcher = QObjectBox::new(Dispatcher::new(tx));
  engine.set_object_property("dispatcher".into(), dispatcher.pinned());
  engine.load_file("qrc:/qml/main.qml".into());
  engine.exec();
  Ok(())
}
