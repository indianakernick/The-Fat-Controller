mod parser;

use tfc::Command;
use std::io::Read;
use parser::{ParseError, parse_tokens};

fn execute(commands: Vec<Command>) -> Result<(), tfc::Error> {
    let mut ctx = tfc::Context::new()?;
    for command in commands.iter() {
        command.execute(&mut ctx)?;
    }
    Ok(())
}

fn main() {
    let mut command_str = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut command_str) {
        println!("Error reading stdin: {}", e);
        return;
    }

    let command_str = command_str.to_ascii_lowercase();
    let commands = match parse_tokens(command_str.split_ascii_whitespace()) {
        Ok(commands) => commands,
        Err(e) => {
            println!("Error parsing commands: {}", e);
            return;
        },
    };

    if let Err(e) = execute(commands) {
        println!("Error executing commands: {}", e);
    }
}
