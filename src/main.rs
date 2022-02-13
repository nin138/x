use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::PathBuf;
use std::process::{Command};

extern crate dirs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "add" {
        add(args);
        return;
    }

    call(args);
}

fn get_conf_path() -> PathBuf {
    let home = dirs::home_dir().unwrap();
    home.join(".xrc")
}

fn get_conf_file() -> File {
    let conf_path = get_conf_path();
    match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open(&conf_path)
    {
        Ok(file) => file,
        Err(e) => error(format!("Couldn't open {}: {}", conf_path.display(), e)),
    }
}

fn add(args: Vec<String>) {
    if args.len() < 4 {
        error(format!("x add format = x add [CMD_NAME] [CMD] ..."))
    }

    let mut file = get_conf_file();

    let mut cmd = String::new();

    cmd.push_str(&args[2]);
    for i in 3..args.len() {
        cmd.push(' ');
        cmd.push_str(&args[i]);
    }
    cmd.push('\n');
    match file.write_all(cmd.as_bytes()) {
        Ok(_) => (),
        Err(e) => error(format!("Couldn't write: {}", e)),
    }
}

fn call(args: Vec<String>) {
    if args.len() < 2 {
        error(format!("usage: x [CMD_NAME] ..."))
    }
    let command = &args[1];
    println!("c: {}", command);
    let file = get_conf_file();

    for result in BufReader::new(file).lines() {
        let line = match result {
            Ok(line) => line,
            Err(e) => error(format!("fail to read line: {}", e)),
        };
        let v: Vec<&str> = line.split(' ').collect();
        if v[0] == command {
            exec(&args, v);
            return;
        }
    }

    error(format!("command not found x {}", command));
}

fn exec(args: &Vec<String>, v: Vec<&str>) {
    let mut command = String::new();
    for i in 1 .. v.len() {
        command.push_str(v[i]);
        command.push(' ');
    }

    for i in 2 .. args.len() {
        command.push_str(&args[i]);
        command.push(' ');
    }


    println!("run: {}", command);
    let c = Command::new(v[1])
        .args(&v[2..v.len()])
        .args(&args[2..args.len()])
        .output()
        .expect("fail to spawn");


    if c.status.success() {
        println!("{}", String::from_utf8(c.stdout).unwrap())
    } else {
        println!("{}", String::from_utf8(c.stdout).unwrap());
        error(String::from_utf8(c.stderr).unwrap());
    }
}

fn error(message: String) -> ! {
    println!("\x1b[93m{}\x1b[0m", message);
    std::process::exit(1)
}