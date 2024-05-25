use std::process;

use crate::parse_input;

pub fn search(input: &str) -> Option<Box<dyn FnOnce(Option<String>)>> {
    match input {
        "exit" => Some(Box::new(|exit_code: Option<String>| {
            process::exit(
                exit_code
                    .unwrap()
                    .parse::<i32>()
                    .expect("exit should be followed by a number"),
            );
        })),
        "echo" => Some(Box::new(|to_echo| {
            let to_echo = to_echo.unwrap_or_default();
            println!("{to_echo}");
        })),
        "type" => Some(Box::new(|args| {
            if let Some(args) = args {
                let (cmd, _) = parse_input(&args);
                if let Some(cmd) = cmd {
                    if search(&cmd).is_some() {
                        println!("{cmd} is a shell builtin")
                    } else {
                        println!("{cmd} not found");
                    }
                }
            } else {
                println!("type must be followed by an argument");
            }
        })),
        _ => None,
    }
}
