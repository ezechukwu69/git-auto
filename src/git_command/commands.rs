use std::{
    io::{self, BufRead, Write},
    process::{self, Child, Command, Stdio},
};

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
    printlines(&mut command).expect("Failed to write errors");
    return Ok(());
}

pub fn exit_application() {
    std::io::stderr()
        .write(b"Error: Failed to execute command")
        .unwrap();
    process::exit(1);
}

pub fn commit(args: &Vec<String>) -> std::io::Result<()> {
    add_files()?;
    let mut binding = Command::new("git");
    let mut message: String = String::new();
    if args.len() > 0 {
        message = args[0].clone();
    } else {
        print!("Enter commit message: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut message).unwrap();
    }
    let mut command = binding.arg("commit").arg("-m").stdout(Stdio::piped());
    if message.len() > 0 {
        command = command.arg(message);
    }
    let mut command = command.spawn()?;
    printlines(&mut command).expect("Failed to write errors");
    Ok(())
}

fn printlines(command: &mut Child) -> io::Result<()> {
    if let Some(stdout) = command.stdout.take() {
        for line in std::io::BufReader::new(stdout).lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn get_only_branch_name(string: String) -> String {
    let split: Vec<&str> = string.split(" ").collect();
    if split.len() > 1 {
        split[1].to_string()
    } else {
        split[0].to_string()
    }
}

pub fn push(args: &Vec<String>) -> std::io::Result<()> {
    let mut branch = String::new();
    if args.len() > 0 {
        branch = args[0].clone();
    } else {
        let branches: Vec<String> = get_branches()?;
        println!("Choose branch:");
        if branches.len() > 0 {
            for (i, b) in branches.iter().enumerate() {
                println!("{}) {}", i + 1, get_only_branch_name(b.to_owned()));
            }
            print!("Enter your choice: ");
            std::io::stdout().flush().unwrap();
            let mut choice: String = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            branch =
                get_only_branch_name(branches[choice.trim().parse::<usize>().unwrap() - 1].clone());
        } else {
            exit_application();
        }
    }

    let mut command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(branch)
        .stdout(Stdio::piped())
        .spawn()?;
    printlines(&mut command).expect("Failed to write errors");
    Ok(())
}

pub fn pull(args: &Vec<String>) -> std::io::Result<()> {
    let mut branch = String::new();
    if args.len() > 0 {
        branch = args[0].clone();
    } else {
        print!("Enter branch name: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut branch).unwrap();
        branch = branch.trim().to_string();
    }
    let mut binding = Command::new("git");
    let command = binding.arg("pull").arg("origin").arg(branch);
    if args.len() > 1 {
       for i in &args[1..] {
           command.arg(i);
       }
    } else {
      command.arg("--ff");
    }
    let mut command = command.stdout(Stdio::piped()).spawn()?;
    printlines(&mut command).expect("Failed to write errors");
    Ok(())
}
