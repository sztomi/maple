use std::sync::{Arc, Mutex};
use qmetaobject::QEnum;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[derive(QEnum)]
#[repr(i32)]
pub enum AppState {
  LoggedOut = 0,
  LoggingIn = 1,
  LoggedIn = 2,
  Error = -1,
}

pub struct App {
  pub app_state: AppState,
}

pub type SharedApp = Arc<Mutex<App>>;

impl App {
  pub fn new(app_state: AppState) -> Self {
    Self {
      app_state,
    }
  }
}
