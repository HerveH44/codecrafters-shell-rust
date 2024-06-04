use std::{env, path::PathBuf, str::Split};

pub fn search_builtin_func(input: &str) -> Option<fn(&mut Split<char>)> {
    match input {
        "exit" => Some(exit),
        "echo" => Some(echo),
        "type" => Some(type_builtin),
        _ => None,
    }
}

fn echo(args: &mut Split<char>) {
    let args = args.collect::<Vec<_>>().join(" ");
    println!("{args}");
}

fn exit(args: &mut Split<char>) {
    if let Some(first_arg) = args.next() {
        if let Ok(exit_code) = first_arg.parse::<i32>() {
            std::process::exit(exit_code);
        }
        println!("exit should be followed by a number. Found {first_arg}");
    } else {
        println!("exit takes at least one argument");
    }
}

fn type_builtin(args: &mut Split<char>) {
    if let Some(cmd) = args.next() {
        if search_builtin_func(cmd).is_some() {
            println!("{cmd} is a shell builtin");
            return;
        }

        if let Some(path) = search_path(cmd) {
            println!("{cmd} is {}", path.display());
            return;
        }
        println!("{cmd} not found");
    } else {
        println!("type should take at least one argument");
    }
}

pub fn search_path(cmd: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|path| path.join(cmd))
        .find(|path| path.exists())
}
