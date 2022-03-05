pub fn bool_to_num(value: bool) -> char {
  match value {
    true => '1',
    false => '2',
  }
}
