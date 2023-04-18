mod commands;
mod shell_command;

use std::io::Write;
use shell_command::ShellCommand;

// Define a struct that represents a shell instance
struct Shell {
    commands: Vec<Box<dyn ShellCommand>>,
}

impl Shell {
    // Creates a new shell instance with the given commands
    fn new(commands: Vec<Box<dyn ShellCommand>>) -> Shell {
        Shell { commands }
    }

    // Starts the shell
    fn start(&self) {
        loop {
            // Print a prompt
            print!("> ");
            std::io::stdout().flush().unwrap();

            // Read user input
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            // Skip empty input
            if input.is_empty() {
                continue;
            }

            // Handle built-in commands
            match input {
                "exit" => return,
                _ => (),
            }

            // Parse the input into a command and arguments
            let mut parts = input.split_whitespace();
            let command = parts.next().unwrap();
            let args = parts.collect::<Vec<_>>();

            // Find the command with the given name and execute it
            for cmd in &self.commands {
                if cmd.name() == command {
                    cmd.execute(&args);
                    break;
                }
            }
        }
    }
}

fn main() {
    let commands: Vec<Box<dyn ShellCommand>> = vec![
        Box::new(commands::echo::EchoCommand {}),
        Box::new(commands::pwd::PwdCommand {}),
        Box::new(commands::cd::CdCommand {}),
        Box::new(commands::ls::LsCommand {}),
        Box::new(commands::cat::CatCommand {})
    ];

    let shell = Shell::new(commands);
    shell.start();
}
