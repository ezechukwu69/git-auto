pub enum GitCommandType {
   Add,
   Commit,
   Push,
   Pull
}


pub struct GitCommand {
    pub git_command_type: GitCommandType,
    pub args: Vec<String>
}


pub fn execute_command(command: &GitCommand) {

}
