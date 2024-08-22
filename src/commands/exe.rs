use std::{env, fs, process, str::from_utf8};
use crate::remove_linebreaks;

pub fn find_executable(split_command: Vec<&str>) {
  let paths: String = env::var("PATH").unwrap();
  
  let found_exe = paths.
  split(":").find(
    |path| fs::metadata(format!("{}/{}",path,split_command[0])).is_ok()
  );

  match found_exe {
    Some(exe) => {
      let output = process::Command::new(split_command[0]).
      arg(split_command[1]).output().expect("Error executing file.");
      let std_out = from_utf8(&output.stdout).expect("Error executing file.");
      println!("{}", remove_linebreaks(std_out.to_string()));     
    },
    None => println!("{}: command not found", split_command[0])
  }
}