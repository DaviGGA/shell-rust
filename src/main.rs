#[allow(unused_imports)]
use std::io::{self, Write};
use shell_starter_rust::*;

fn main() {

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = remove_linebreaks(input);
        
        execute_command(input.as_str());      
    }

}
