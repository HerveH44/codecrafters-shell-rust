pub mod builtins;

use std::io::{self, Write};
use std::process::Command;
use std::str::Split;

use builtins::{search_builtin_func, search_path};

fn main() {
    loop {
        print_prompt();
        let input = get_input();
        let mut args = input.split(' ');
        if let Some(cmd) = args.next() {
            handle_command(cmd, args);
        }
    }
}

fn handle_command(cmd: &str, mut args: Split<char>) {
    if let Some(builtin_func) = search_builtin_func(cmd) {
        builtin_func(&mut args);
    } else if search_path(cmd).is_some() {
        run_program(cmd, &mut args);
    } else {
        println!("{cmd}: command not found");
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

fn run_program(cmd: &str, args: &mut Split<char>) {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to run the program");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
