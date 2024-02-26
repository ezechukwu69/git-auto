use super::commands;

#[derive(Debug)]
pub enum GitCommandType {
   Add,
   Commit,
   Push,
   Pull,
   ListBranches,
   Help
}

#[derive(Debug)]
pub struct GitCommand {
    pub git_command_type: GitCommandType,
    pub args: Vec<String>
}

pub fn execute_command(command: &GitCommand) {
    match command.git_command_type {
        GitCommandType::ListBranches => commands::list_branches(),
        GitCommandType::Commit => commands::commit(&command.args).expect("Failed to commit"),
        GitCommandType::Add => commands::add_files().expect("Failed to add files"),
        GitCommandType::Push => commands::push(&command.args).expect("Failed to push"),
        GitCommandType::Pull => commands::pull(&command.args).expect("Failed to pull"),
        _ => commands::exit_application()
    }
}


