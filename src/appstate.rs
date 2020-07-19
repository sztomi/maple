use std::sync::{Arc, Mutex};

pub enum LoginState {
  LoggedOut,
  LoggingIn,
  LoggedIn,
  Error
}

pub struct App {
  login_state: LoginState,
}

pub type SharedApp = Arc<Mutex<App>>;

impl App {
  pub fn new(login_state: LoginState) -> Self {
    Self {
      login_state: login_state,
    }
  }
}
