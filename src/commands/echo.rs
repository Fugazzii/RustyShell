use crate::shell_command::ShellCommand;

// Define a struct that implements the ShellCommand trait for the "echo" command
pub struct EchoCommand;
impl ShellCommand for EchoCommand {
    fn name(&self) -> &str {
        "echo"
    }

    fn execute(&self, args: &[&str]) {
        println!("{}", args.join(" "));
    }
}
