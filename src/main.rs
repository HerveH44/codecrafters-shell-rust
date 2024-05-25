pub mod builtins;

#[allow(unused_imports)]
use std::io::{self, Write};

use crate::builtins::search;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let (cmd, args) = parse_input(input.trim());
        if cmd.is_none() {
            continue;
        }
        let cmd = cmd.unwrap();

        if let Some(func) = search(&cmd) {
            func(args);
        } else {
            println!("{cmd}: command not found");
        }
    }
}

pub fn parse_input(input: &str) -> (Option<String>, Option<String>) {
    if input.is_empty() {
        return (None, None);
    }

    let split_input = input.split_once(' ');
    match split_input {
        None => (Some(input.to_owned()), None),
        Some((cmd, args)) => (Some(cmd.to_owned()), Some(args.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn can_parse_command_and_return_args() {
        let (cmd, args) = parse_input("exit 0");

        assert_eq!(Some("exit".to_owned()), cmd);
        assert_eq!(Some("0".to_owned()), args);
    }

    #[test]
    fn if_no_command_exists_should_return_none() {
        let (cmd, args) = parse_input("");

        assert_eq!(None, cmd);
        assert_eq!(None, args);
    }
}
