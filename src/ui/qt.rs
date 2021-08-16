use std::sync::mpsc::Sender;

use anyhow::Result;
use cpp::cpp;
use qmetaobject::prelude::*;
use log;

use crate::appstate::{SharedApp, AppState};
use crate::client::NetworkEvent;
use super::dispatcher::Dispatcher;

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


pub fn run_ui<'a>(app: &'a SharedApp, tx: Sender<NetworkEvent>) -> Result<()> {
  tx.send(NetworkEvent::Startup)?;
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
