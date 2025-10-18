use reedline::{DefaultPrompt, Reedline, Signal};
use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

fn command_successful(buffer: &str, path_files: &HashMap<String, String>) -> bool {
    let parts: Vec<&str> = buffer.split_ascii_whitespace().collect();

    if let Some((&command, arguments)) = parts.split_first() {
        if command == "cd" {
            let directory = arguments[0];
            let success = common::set_current_directory(directory);

            if !success {
                print!("No such file or directory");
            }

            return true;
        }

        if let Some(command) = path_files.get(command) {
            let _ = Command::new(command)
                .args(arguments)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            return true;
        }

        println!("Command not found");
    }

    true
}

pub fn command(
    reedline: &mut Reedline,
    prompt: &DefaultPrompt,
    path_files: &HashMap<String, String>,
) -> bool {
    match reedline.read_line(prompt) {
        Ok(Signal::Success(buffer)) => command_successful(&buffer, path_files),
        _ => false,
    }
}
