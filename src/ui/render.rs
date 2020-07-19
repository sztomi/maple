use std::sync::mpsc::Sender;

use crate::ui::scaffolding;
use crate::network::NetworkEvent;
use crate::appstate::SharedApp;

use anyhow::Result;

#[macro_use]
use imgui::*;

pub fn run_ui<'a>(app: &'a SharedApp, tx: &Sender<NetworkEvent>) -> Result<()> {
  let window_title = im_str!("Maple for Plex");
  let system = scaffolding::init(&window_title.to_string());
  let window_size = system.render_sys.window().get_inner_size().unwrap();

  system.main_loop(move |_, ui| {
    Window::new(window_title)
      .position([0.0, 0.0], Condition::Always)
      .size([300.0, window_size.height as f32], Condition::FirstUseEver)
      .title_bar(false)
      .movable(false)
      .resizable(false)
      .build(ui, || {
        ui.text(format!("FPS: {}", ui.io().framerate));
        if ui.button(im_str!("Log in"), [80.0, 20.0]) {
          tx.send(NetworkEvent::Login);
        }
        ()
      });
  });
  Ok(())
}
