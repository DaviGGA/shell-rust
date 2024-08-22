pub fn echo(command: &str) {
  const ECHO_LAST_CHAR_IDX: usize = 5;
  println!("{}", &command[ECHO_LAST_CHAR_IDX..]);
}