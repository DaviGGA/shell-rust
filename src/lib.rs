mod commands_enum;
mod commands;

use std::{env, ffi::OsString, path, process, str::{from_utf8, FromStr}};
use commands_enum::Commands;
use commands::{
  echo,
  exit, 
  type_c, 
  pwd, 
  cd, 
  find_executable
};

pub fn execute_command(command: &str) -> () {
  let split_command = command.split(' ').collect::<Vec<&str>>();
  let first_command: &str = split_command[0];

  match Commands::from_str(first_command) {
      Ok(Commands::Exit) => exit(command),
      Ok(Commands::Echo) => echo(command),
      Ok(Commands::Type) => type_c(command),
      Ok(Commands::Pwd) => pwd(command),
      Ok(Commands::Cd) => cd(command),
      _ => find_executable(split_command)
  };
}

pub fn remove_linebreaks(text: String) -> String {
  text.
    strip_suffix("\r\n").
    or(text.strip_suffix("\n")).
    unwrap_or(text.as_str()).
    to_string()
}



