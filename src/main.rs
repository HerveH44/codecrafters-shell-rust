#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();

        if command.starts_with("exit ") {
            if let Some((_, exit_code)) = command.split_once(' ') {
                process::exit(exit_code.parse().unwrap());
            }
        }

        if command.starts_with("echo ") {
            if let Some((_, to_echo)) = command.split_once(' ') {
                println!("{to_echo}");
            }
        } else {
            println!("{command}: command not found");
        }
    }
}
