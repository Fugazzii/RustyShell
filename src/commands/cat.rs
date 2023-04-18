use crate::shell_command::ShellCommand;

pub struct CatCommand {}

impl ShellCommand for CatCommand {
    fn name(&self) -> &str {
        "cat"
    }

    fn execute(&self, args: &[&str]) {
        let output = std::process::Command::new("cat").args(args).output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        print!("{}", stdout);

        if !stdout.ends_with('\n') {
            println!();
        }
    }
}
