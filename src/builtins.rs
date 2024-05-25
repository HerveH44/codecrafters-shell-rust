use std::{env, path::PathBuf};

pub fn search_builtin_func(input: &str) -> Option<fn(Vec<String>)> {
    match input {
        "exit" => Some(exit),
        "echo" => Some(echo),
        "type" => Some(type_builtin),
        _ => None,
    }
}

fn echo(args: Vec<String>) {
    let args = args.join(" ");
    println!("{args}");
}

fn exit(args: Vec<String>) {
    if args.is_empty() {
        println!("exit takes at least one argument");
        return;
    }

    let exit_code = &args[0];
    std::process::exit(
        exit_code
            .parse::<i32>()
            .expect("exit should be followed by a number"),
    );
}

fn type_builtin(args: Vec<String>) {
    if args.is_empty() {
        println!("type should take at one argument");
        return;
    }

    let cmd = &args[0];
    if search_builtin_func(cmd).is_some() {
        println!("{cmd} is a shell builtin");
        return;
    }

    if let Some(path) = search_path(cmd) {
        println!("{cmd} is {}", path.display());
        return;
    }
    println!("{cmd} not found");
}

pub fn search_path(cmd: &str) -> Option<PathBuf> {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let possible_path = path.join(cmd);
            if possible_path.exists() {
                return Some(possible_path);
            }
        }
    }
    None
}
