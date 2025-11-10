use std::{
    env::{self},
    path::{Path, PathBuf},
    str::Split,
};

pub fn search_builtin_func(input: &str) -> Option<fn(&mut Split<char>)> {
    match input {
        "exit" => Some(exit),
        "echo" => Some(echo),
        "type" => Some(type_builtin),
        "pwd" => Some(pwd),
        "cd" => Some(cd),
        _ => None,
    }
}

fn echo(args: &mut Split<char>) {
    let args = args.collect::<Vec<_>>().join(" ");

    // remove single quotes
    let args: String = args.split("'").collect();

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

fn pwd(_args: &mut Split<char>) {
    match env::current_dir() {
        Ok(current_dir) => {
            println!("{}", current_dir.display());
        }
        Err(_) => {
            println!("could not find current dir");
        }
    }
}

fn cd(args: &mut Split<char>) {
    let path_string = match args.next().unwrap() {
        "~" => {
            if let Some(home_path) = env::var_os("HOME") {
                home_path.to_os_string().into_string().unwrap()
            } else {
                println!("could not find HOME env var");
                return;
            }
        }
        other => other.to_string(),
    };
    change_directory(&path_string);
}

fn change_directory(path: &str) {
    if env::set_current_dir(Path::new(&path)).is_err() {
        println!("{path}: No such file or directory");
    };
}

pub fn search_path(cmd: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|path| path.join(cmd))
        .find(|path| path.exists())
}
