pub mod builtins;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

use crate::builtins::{search_builtin_func, search_path};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Parse input
        let (cmd, args) = parse_input(input.trim());

        // nothing written
        if cmd.is_none() {
            continue;
        }

        // Check if built in command knows it
        let cmd = cmd.unwrap();
        if let Some(builtin_func) = search_builtin_func(&cmd) {
            builtin_func(args);
        } else if search_path(&cmd).is_some() {
            let output = Command::new(cmd)
                .args(args)
                .output()
                .expect("Failed to run the program");
            println!("status: {}", output.status);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        } else {
            println!("{cmd}: command not found");
        }
    }
}

pub fn parse_input(input: &str) -> (Option<String>, Vec<String>) {
    if input.is_empty() {
        return (None, vec![]);
    }

    let split_input = input.split_once(' ');
    match split_input {
        None => (Some(input.to_owned()), vec![]),
        Some((cmd, args)) => (
            Some(cmd.to_owned()),
            args.split(' ').map(|s| s.to_owned()).collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn can_parse_command_and_return_args() {
        let (cmd, args) = parse_input("exit 0");

        assert_eq!(Some("exit".to_owned()), cmd);
        assert_eq!(vec!["0"], args);
    }

    #[test]
    fn if_no_command_exists_should_return_none() {
        let (cmd, args) = parse_input("");

        assert_eq!(None, cmd);
        assert_eq!(vec![] as Vec<String>, args);
    }
}
