use std::env;

pub fn pwd(command: &str) {
  let absolute_path = env::current_dir();

  match absolute_path {
    Ok(path) => println!("{}", path.display()),
    Err(err) => println!("PwdError: {err}")
  }
}