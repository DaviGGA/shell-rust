use std::process;

pub fn exit(command: &str) {
  let command_split = command.split(" ").collect::<Vec<&str>>();

  let status_code_str = command_split.
    get(1).
    unwrap_or(&"0");

  match status_code_str.parse::<i32>() {
    Ok(code) => process::exit(code),
    Err(err) => println!("Exit command error: {err}")
  }
}