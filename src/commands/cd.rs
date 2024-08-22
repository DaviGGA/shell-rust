use std::{env, ffi::OsString, path::{self, Path, PathBuf}, usize};
use crate::remove_linebreaks;

pub fn cd (command: &str) {
  
  const CD_LAST_CHAR_IDX: usize = 3;

  if command.len() < 3 as usize {
    return
  }
  
  let mut cd_path = path::PathBuf::new();
  cd_path.push(&command[CD_LAST_CHAR_IDX..]);

  let is_tilde = *&command[CD_LAST_CHAR_IDX..].trim() == '~'.to_string().as_str();

  // TO-DO: this only works in linux or windows, also it needs better error handling.
  let home_env: String = match std::env::consts::OS {
    "linux" => env::var("HOME").unwrap(),
    "windows" => env::var("HOMEPATH").unwrap(),
    _ => String::new()
  };

  if is_tilde {
    let _ = env::set_current_dir(Path::new(&home_env)).unwrap();
    return
  }
  
  let absolute_path = env::current_dir().unwrap();
  let new_path = absolute_path.join(cd_path);

  set_dir(new_path, &command[CD_LAST_CHAR_IDX..]);
}

fn set_dir(path: PathBuf, path_str: &str) {
  match env::set_current_dir(path) {
    Ok(_) => {},
    Err(err) => println!("cd: {}: No such file or directory",remove_linebreaks(path_str.to_string()))
  }
}

