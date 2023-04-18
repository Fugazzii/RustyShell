use std::fs;
use crate::shell_command::ShellCommand;

pub struct LsCommand;

impl ShellCommand for LsCommand {
    fn execute(&self, args: &[&str]) {
        let dir_path = if args.is_empty() {
            ".".to_string()
        } else {
            args[0].to_string()
        };

        let dir = match fs::read_dir(dir_path) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Error listing directory: {}", e);
                return;
            }
        };

        for entry in dir {
            match entry {
                Ok(entry) => {
                    let file_name = entry.file_name();
                    let file_type = match entry.file_type() {
                        Ok(file_type) => file_type,
                        Err(e) => {
                            eprintln!("Error getting file type: {}", e);
                            continue;
                        }
                    };
                    let file_type_str = if file_type.is_dir() {
                        "dir"
                    } else {
                        "file"
                    };
                    println!("{} ({})", file_name.to_string_lossy(), file_type_str);
                }
                Err(e) => {
                    eprintln!("Error iterating over directory entries: {}", e);
                    continue;
                }
            }
        }
    }

    fn name(&self) -> &str {
        "ls"
    }
}
