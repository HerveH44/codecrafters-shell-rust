pub mod builtins;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

use crate::builtins::{search_builtin_func, search_path};

fn main() {
    loop {
        print_prompt();
        let input = get_input();
        let (cmd, args) = parse_input(input.trim());

        if let Some(cmd) = cmd {
            if let Some(builtin_func) = search_builtin_func(cmd) {
                builtin_func(args);
            } else if search_path(cmd).is_some() {
                run_program(cmd, args);
            } else {
                println!("{cmd}: command not found");
            }
        }
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}

fn run_program(cmd: &str, args: Vec<&str>) {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to run the program");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub fn parse_input(input: &str) -> (Option<&str>, Vec<&str>) {
    if input.is_empty() {
        return (None, vec![]);
    }

    let split_input = input.split_once(' ');
    match split_input {
        None => (Some(input), vec![]),
        Some((cmd, args)) => (Some(cmd), args.split(' ').collect()),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn can_parse_command_and_return_args() {
        let (cmd, args) = parse_input("exit 0");

        assert_eq!(Some("exit"), cmd);
        assert_eq!(vec!["0"], args);
    }

    #[test]
    fn if_no_command_exists_should_return_none() {
        let (cmd, args) = parse_input("");

        assert_eq!(None, cmd);
        assert_eq!(vec![] as Vec<String>, args);
    }
}
