use std::sync::{Arc, Mutex};
use qmetaobject::QEnum;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[derive(QEnum)]
#[repr(i32)]
pub enum LoginState {
  LoggedOut = 0,
  LoggingIn = 1,
  LoggedIn = 2,
  Error = -1,
}

pub struct App {
  pub login_state: LoginState,
}

pub type SharedApp = Arc<Mutex<App>>;

impl App {
  pub fn new(login_state: LoginState) -> Self {
    Self {
      login_state,
    }
  }
}
