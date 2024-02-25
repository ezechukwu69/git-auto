use git_command::git_automator;

mod git_command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    git_automator::automate_git(args.clone()[0..].to_owned())
}
