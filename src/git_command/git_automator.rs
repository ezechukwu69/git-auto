use std::{io::Write, process};

use super::git_command::{GitCommandType, GitCommand, execute_command};

pub fn automate_git(args: Vec<String>) {
   let command: GitCommandType = match args.len() {
       0 => { ask_command() }
       _ => choose_command(&args[0])
    };
    let git_command = GitCommand {
        git_command_type: command,
        args
    };

    execute_command(&git_command);
}

fn ask_command() -> GitCommandType {
    return match std::io::stdout().write(b"
        Choose an option:
        1) Add
        2) Commit
        3) Push
        4) Pull
        ") {
        Ok(_) => {
            print!("Enter command number: ");
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).unwrap();
            let split_command = command.trim().split(" ");
            if split_command.clone().count() == 0 {
                std::io::stderr().write(b"Error: Invalid command").unwrap();
                process::exit(1);
            } else {
                choose_command(split_command.clone().nth(0).unwrap())
            }
        },
        Err(_) => {
             std::io::stderr().write(b"Error: Failed to write to stdout").unwrap();
             process::exit(1);
        }
    };
}

fn choose_command(command: &str) -> GitCommandType {
    return match command {
        "1" => GitCommandType::Add,
        "2" => GitCommandType::Commit,
        "3" => GitCommandType::Push,
        "4" => GitCommandType::Pull,
        _ => {
            std::io::stderr().write(b"Error: Invalid command").unwrap();
            process::exit(1);
        }
    }
}
