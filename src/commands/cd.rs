use crate::shell_command::ShellCommand;

// Changes the current working directory
pub struct CdCommand;

impl ShellCommand for CdCommand {
    fn name(&self) -> &str {
        "cd"
    }

    fn execute(&self, args: &[&str]) {
        let new_dir = args.get(0).map_or("/", |dir| dir);

        if let Err(err) = std::env::set_current_dir(new_dir) {
            eprintln!("cd: {}: {}", new_dir, err);
        }
    }
}