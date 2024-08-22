use std::{env, fs, str::FromStr};
use crate::Commands;

pub fn type_c(command: &str) {
  const TYPE_LAST_CHAR_IDX: usize = 5;
  
  let type_command = &command[TYPE_LAST_CHAR_IDX..];

  match Commands::from_str(type_command) {
      Ok(_) => println!("{type_command} is a shell builtin"),
      Err(_) => check_type_path(type_command)
  }

}

fn check_type_path(type_command: &str) {
  let paths: String = env::var("PATH").unwrap();

  let found_path = paths.
  split(":").find(
    |path| fs::metadata(format!("{}/{}",path,type_command)).is_ok()
  );

  match found_path {
    Some(path) => println!("{} is {}", type_command, format!("{}/{}",path,type_command)),
    None => println!("{type_command}: not found")
  }
}