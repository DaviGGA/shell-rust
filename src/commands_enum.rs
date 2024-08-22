
use std::str::FromStr;
pub enum Commands {
  Exit,
  Echo,
  Type,
  Pwd,
  Cd
}

impl FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "exit" => Ok(Commands::Exit),
          "echo" => Ok(Commands::Echo),
          "type" => Ok(Commands::Type),
          "pwd" => Ok(Commands::Pwd),
          "cd" => Ok(Commands::Cd),
          _ => Err(s.to_string())
        }
    }
}