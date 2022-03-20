
#[derive(Debug)]
pub enum AppEvent {
  Started,
  LoginRequested,
  LogoutRequested,
  MenuItemClicked(i32),
  //LoggedIn,
}