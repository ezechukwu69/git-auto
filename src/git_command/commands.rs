use std::{process::{Stdio, Command, self}, io::{BufRead, Write, self}};

pub fn list_branches() {
    println!("Here is a list of branches available");
    if let Ok(branches) = get_branches() {
        for branch in branches {
            println!("{}", branch);
        }
    }
}

pub fn get_branches() -> std::io::Result<Vec<String>> {
    let mut command = Command::new("git")
        .arg("branch")
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = command.stdout.take().unwrap();

    let mut branches = Vec::new();
    for line in std::io::BufReader::new(stdout).lines() {
        branches.push(line?);
    }

    if command.wait()?.success() {
        return Ok(branches);
    } else {
        return Ok(Vec::new());
    }
}

pub fn add_files() -> std::io::Result<()> {
    let mut command = Command::new("git")
        .arg("add")
        .arg("-A")
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = command.stdout.take().unwrap();

    let mut stdout_reader = std::io::BufReader::new(stdout);
    let mut line = String::new();
    while stdout_reader.read_line(&mut line)? > 0 {
        println!("{}", line);
        line.clear();
    }

    return Ok(());
}

pub fn exit_application() {
        std::io::stderr().write(b"Error: Failed to execute command").unwrap();
        process::exit(1);
}


pub fn commit(args: &Vec<String>) -> std::io::Result<()> {
    let mut binding = Command::new("git");
    let mut message: String = String::new();
    if args.len() > 0 {
        message = args[0].clone();
    } else {
        print!("Enter commit message: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut message).unwrap();
    }
    let mut command = binding
        .arg("commit")
        .arg("-m");
    if message.len() > 0 {
        command = command.arg(message);
    }
    let mut command = command.spawn()?;
    let stdout = command.stdout.take().unwrap();
    for line in std::io::BufReader::new(stdout).lines() {
      println!("{}", line?);
    }
    Ok(())
}
