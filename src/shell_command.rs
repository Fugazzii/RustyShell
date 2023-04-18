// Define a trait that encapsulates the logic for a shell command
pub trait ShellCommand {
    // Returns the name of the command
    fn name(&self) -> &str;

    // Executes the command with the given arguments
    fn execute(&self, args: &[&str]);
}
