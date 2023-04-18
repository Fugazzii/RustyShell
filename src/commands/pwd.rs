use crate::shell_command::ShellCommand;

// Define a struct that implements the ShellCommand trait for the "pwd" command
pub struct PwdCommand;
impl ShellCommand for PwdCommand {
    fn name(&self) -> &str {
        "pwd"
    }

    fn execute(&self, _args: &[&str]) {
        let path = std::env::current_dir().unwrap();
        println!("{}", path.display());
    }
}
