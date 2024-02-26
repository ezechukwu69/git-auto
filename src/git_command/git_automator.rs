use std::{io::Write, process};

use super::git_command::{execute_command, GitCommand, GitCommandType};

pub fn automate_git(args: Vec<String>) {
    let command: GitCommandType = match args.len() {
        0 => ask_command(),
        _ => choose_command_string(&args[0]),
    };
    let git_command = GitCommand {
        git_command_type: command,
        args: if args.len() > 1 {
            args[1..].to_vec().to_owned()
        } else {
            Vec::new()
        },
    };

    execute_command(&git_command);
}

fn ask_command() -> GitCommandType {
    print!(
        "
Choose an option:
1) Add
2) Commit
3) Push
4) Pull
5) List branches
6) Help

Enter your choice: "
    );
    let mut command = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut command).unwrap();
    let split_command = command.trim().split(" ");
    if split_command.clone().count() == 0 {
        std::io::stderr().write(b"Error: Invalid command").unwrap();
        process::exit(1);
    } else {
        return choose_command(split_command.clone().nth(0).unwrap());
    }
}

fn choose_command(command: &str) -> GitCommandType {
    return match command {
        "1" => GitCommandType::Add,
        "2" => GitCommandType::Commit,
        "3" => GitCommandType::Push,
        "4" => GitCommandType::Pull,
        "5" => GitCommandType::ListBranches,
        "6" => GitCommandType::ListBranches,
        _ => {
            std::io::stderr().write(b"Error: Invalid command").unwrap();
            process::exit(1);
        }
    };
}

fn choose_command_string(command: &str) -> GitCommandType {
    return match command {
        "add" => GitCommandType::Add,
        "commit" => GitCommandType::Commit,
        "push" => GitCommandType::Push,
        "pull" => GitCommandType::Pull,
        "list-branches" => GitCommandType::ListBranches,
        "help" => GitCommandType::Help,
        _ => {
            std::io::stderr().write(b"Error: Invalid command").unwrap();
            process::exit(1);
        }
    };
}
